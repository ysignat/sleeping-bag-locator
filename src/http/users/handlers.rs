use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router,
};
use uuid::Uuid;

use super::{
    common::AppError,
    dtos::{HttpCreateUserParams, HttpUpdateUserParams, HttpUser},
    state::AppState,
};

#[derive(Default)]
pub struct UserRouter {}

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(params): Json<HttpCreateUserParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpUser = state.users.create(params.into()).await?.into();

    Ok((StatusCode::CREATED, Json(result)))
}

#[debug_handler]
pub async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpUser = state.users.get(id).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(params): Json<HttpUpdateUserParams>,
) -> Result<impl IntoResponse, AppError> {
    let result: HttpUser = state.users.update(id, params.into()).await?.into();

    Ok((StatusCode::OK, Json(result)))
}

#[debug_handler]
pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    state.users.delete(id).await?;

    Ok(StatusCode::NO_CONTENT)
}

impl From<UserRouter> for Router<AppState> {
    fn from(_: UserRouter) -> Self {
        Router::new()
            .route("/", post(create_user))
            .route("/:id", get(get_user).put(update_user).delete(delete_user))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_session::{
        serde_json::{self, from_slice},
        MemoryStore,
    };
    use axum::{body::Body, http::Request};
    use fake::{Fake, Faker};
    use http_body_util::BodyExt;
    use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
    use reqwest::{header::CONTENT_TYPE, Method, Url};
    use rstest::rstest;
    use serde_json::to_string;
    use tower::ServiceExt;

    use super::*;
    use crate::dao::{CreateUserParams, ItemsMockedDao, UsersDao, UsersHashMapDao};

    impl Default for AppState {
        fn default() -> Self {
            let localhost = Url::parse("http://localhost/").unwrap();

            Self {
                items: Arc::new(ItemsMockedDao {}),
                users: Arc::new(UsersHashMapDao::new()),
                session_store: Arc::new(MemoryStore::new()),
                oauth: BasicClient::new(ClientId::new(String::new()))
                    .set_client_secret(ClientSecret::new(String::new()))
                    .set_auth_uri(AuthUrl::new(localhost.to_string()).unwrap())
                    .set_token_uri(TokenUrl::new(localhost.to_string()).unwrap()),
            }
        }
    }

    #[tokio::test]
    async fn create_ok() {
        let router: Router<AppState> = UserRouter::default().into();

        let params = Faker.fake::<HttpCreateUserParams>();
        println!("{params:#?}");

        let raw_response = router
            .with_state(AppState::default())
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/")
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::CREATED);

        let response =
            from_slice::<HttpUser>(&raw_response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        println!("{response:#?}");

        assert_eq!(params.name(), response.name());
        assert_eq!(params.auth_type(), response.auth_type());
        assert_eq!(params.external_id(), response.external_id());
        assert_eq!(response.created_at(), response.updated_at());
    }

    #[rstest]
    #[case::no_name("name")]
    #[case::no_auth_type("auth_type")]
    #[case::no_external_id("external_id")]
    #[tokio::test]
    async fn create_param_not_set(#[case] param_not_set: &str) {
        let router: Router<AppState> = UserRouter::default().into();

        let mut params = serde_json::to_value(Faker.fake::<HttpCreateUserParams>())
            .unwrap()
            .as_object()
            .unwrap()
            .to_owned();

        params.remove(param_not_set);
        println!("{params:#?}");

        let raw_response = router
            .with_state(AppState::default())
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/")
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[rstest]
    #[case::empty_name("name", "")]
    #[case::long_name("name", (129..256).fake())]
    #[case::empty_external_id("external_id", "")]
    #[case::long_external_id("external_id", (129..256).fake())]
    #[case::junk_auth_type("auth_type", "foobarbuzz")]
    #[tokio::test]
    async fn create_invalid_param(#[case] name: &str, #[case] value: String) {
        let router: Router<AppState> = UserRouter::default().into();

        let mut params = serde_json::to_value(Faker.fake::<HttpCreateUserParams>())
            .unwrap()
            .as_object()
            .unwrap()
            .to_owned();

        params.insert(name.to_owned(), serde_json::Value::String(value));
        println!("{params:#?}");

        let raw_response = router
            .with_state(AppState::default())
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/")
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn get_ok() {
        let router: Router<AppState> = UserRouter::default().into();
        let params = Faker.fake::<CreateUserParams>();
        println!("{params:#?}");

        let predefined_dao = UsersHashMapDao::new();
        let entity = predefined_dao.create(params.clone()).await.unwrap();

        let state = AppState {
            users: Arc::new(predefined_dao),
            ..Default::default()
        };

        let raw_response = router
            .with_state(state)
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/{}", entity.id()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::OK);

        let response =
            from_slice::<HttpUser>(&raw_response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        println!("{response:#?}");

        assert_eq!(params.name(), response.name());
        assert_eq!(params.auth_type(), response.auth_type().into());
        assert_eq!(params.external_id(), response.external_id());
        assert_eq!(response.created_at(), response.updated_at());
    }

    #[tokio::test]
    async fn get_non_existent() {
        let router: Router<AppState> = UserRouter::default().into();

        let raw_response = router
            .with_state(AppState::default())
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri(format!("/{}", Uuid::new_v4()))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_ok() {
        let router: Router<AppState> = UserRouter::default().into();
        let create_params = Faker.fake::<CreateUserParams>();
        println!("{create_params:#?}");

        let predefined_dao = UsersHashMapDao::new();
        let entity = predefined_dao.create(create_params).await.unwrap();
        let state = AppState {
            users: Arc::new(predefined_dao),
            ..Default::default()
        };

        let update_params = Faker.fake::<HttpUpdateUserParams>();
        println!("{update_params:#?}");

        let raw_response = router
            .with_state(state)
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri(format!("/{}", entity.id()))
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&update_params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::OK);

        let response =
            from_slice::<HttpUser>(&raw_response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        println!("{response:#?}");

        assert_eq!(update_params.name(), response.name());
        assert_eq!(entity.auth_type(), response.auth_type().into());
        assert_eq!(entity.external_id(), response.external_id());
        assert!(response.created_at().le(&response.updated_at()));
    }

    #[rstest]
    #[case::no_name("name")]
    #[tokio::test]
    async fn update_param_not_set(#[case] param_not_set: &str) {
        let router: Router<AppState> = UserRouter::default().into();
        let create_params = Faker.fake::<CreateUserParams>();
        println!("{create_params:#?}");

        let predefined_dao = UsersHashMapDao::new();
        let entity = predefined_dao.create(create_params).await.unwrap();

        let state = AppState {
            users: Arc::new(predefined_dao),
            ..Default::default()
        };

        let mut update_params = serde_json::to_value(Faker.fake::<HttpUpdateUserParams>())
            .unwrap()
            .as_object()
            .unwrap()
            .to_owned();

        update_params.remove(param_not_set);
        println!("{update_params:#?}");

        let raw_response = router
            .with_state(state)
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri(format!("/{}", entity.id()))
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&update_params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[rstest]
    #[case::empty_name("name", "")]
    #[case::long_name("name", (129..256).fake())]
    #[tokio::test]
    async fn update_invalid_param(#[case] name: &str, #[case] value: String) {
        let router: Router<AppState> = UserRouter::default().into();
        let create_params = Faker.fake::<CreateUserParams>();
        println!("{create_params:#?}");

        let predefined_dao = UsersHashMapDao::new();
        let entity = predefined_dao.create(create_params).await.unwrap();

        let state = AppState {
            users: Arc::new(predefined_dao),
            ..Default::default()
        };
        let mut update_params = serde_json::to_value(Faker.fake::<HttpUpdateUserParams>())
            .unwrap()
            .as_object()
            .unwrap()
            .to_owned();

        update_params.insert(name.to_owned(), serde_json::Value::String(value));
        println!("{update_params:#?}");

        let raw_response = router
            .with_state(state)
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri(format!("/{}", entity.id()))
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&update_params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn update_non_existent() {
        let router: Router<AppState> = UserRouter::default().into();
        let params = Faker.fake::<HttpUpdateUserParams>();

        let raw_response = router
            .with_state(AppState::default())
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri(format!("/{}", Uuid::new_v4()))
                    .header(CONTENT_TYPE, "application/json")
                    .body(to_string(&params).unwrap())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(raw_response.status(), StatusCode::NOT_FOUND);
    }
}

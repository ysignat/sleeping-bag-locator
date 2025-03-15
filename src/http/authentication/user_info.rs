use async_session::SessionStore;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use super::{COOKIE_NAME, USER_INFO};
use crate::http::AppState;

const AUTH_PATH: &str = "/auth";

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary(AUTH_PATH).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    id: usize,
}

#[async_trait]
impl<T> FromRequestParts<AppState<T>> for UserInfo
where
    T: SessionStore,
{
    type Rejection = AuthRedirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState<T>,
    ) -> Result<Self, Self::Rejection> {
        let store = state.session_store.clone();

        let cookie_jar = parts.extract::<CookieJar>().await.unwrap(); // Unwrapping Infallible, it's OK
        let session_cookie = cookie_jar.get(COOKIE_NAME).ok_or(AuthRedirect)?;
        let session = store
            .load_session(session_cookie.to_string())
            .await
            .or(Err(AuthRedirect))?
            .ok_or(AuthRedirect)?;

        let user = session.get::<UserInfo>(USER_INFO).ok_or(AuthRedirect)?;

        Ok(user)
    }
}

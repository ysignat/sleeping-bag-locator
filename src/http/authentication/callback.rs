use async_session::{Session, SessionStore};
use axum::{
    extract::{Query, State},
    http::{header::USER_AGENT, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use oauth2::{AuthorizationCode, CsrfToken, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;
use tracing::{error, trace};

use super::{user_info::UserInfo, COOKIE_NAME, CSRF_TOKEN, USER_INFO};
use crate::http::{common::AppError, AppState};

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    code: String,
    state: String,
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
enum AuthCallbackError {
    #[error("No such cookie {cookie_name:?} in a jar")]
    NoSuchCookie { cookie_name: String },
    #[error("Error while fetching session from storage")]
    SessionLoadingFailed,
    #[error("Session collected, but it's empty")]
    EmptySession,
    #[error("Cannot deserialize CSRF token")]
    CsrfTokenDeserializationError,
    #[error("Cannot destroy session with CSRF token")]
    CsrfTokenSessionDestructionError,
    #[error("CSRF token from session diverges from one from OAuth provider")]
    CsrfTokensMismatch,
    #[error("Cannot exchange code for token")]
    CodeExchangeError,
    #[error("Error while sending request for User Info")]
    UserInfoRequestError,
    #[error("Cannot deserialize User Info response")]
    UserInfoDeserializeResponseError,
    #[error("Cannot serialize User Info")]
    UserInfoSerializationError,
    #[error("Cannot store User Info in session storage")]
    UserInfoStorageError,
    #[error("User Info was put in session storage, but returned empty cookie string")]
    UserInfoStorageEmptyCookie,
}

impl From<AuthCallbackError> for AppError {
    fn from(value: AuthCallbackError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            AuthCallbackError::NoSuchCookie { cookie_name: _ }
            | AuthCallbackError::SessionLoadingFailed
            | AuthCallbackError::EmptySession
            | AuthCallbackError::CsrfTokenDeserializationError
            | AuthCallbackError::CsrfTokenSessionDestructionError
            | AuthCallbackError::CsrfTokensMismatch
            | AuthCallbackError::CodeExchangeError
            | AuthCallbackError::UserInfoRequestError
            | AuthCallbackError::UserInfoDeserializeResponseError
            | AuthCallbackError::UserInfoSerializationError
            | AuthCallbackError::UserInfoStorageError
            | AuthCallbackError::UserInfoStorageEmptyCookie => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

pub async fn auth_callback<T>(
    cookie_jar: CookieJar,
    State(state): State<AppState<T>>,
    Query(query): Query<AuthCallbackQuery>,
) -> Result<impl IntoResponse, AppError>
where
    T: SessionStore,
{
    // Get session ID cookie
    let cookie = cookie_jar
        .get(COOKIE_NAME)
        .ok_or(AuthCallbackError::NoSuchCookie {
            cookie_name: COOKIE_NAME.to_owned(),
        })?
        .to_string()
        .split_once('=')
        .unwrap()
        .1
        .to_owned();

    // Collect session from session store
    let session = state
        .session_store
        .load_session(cookie)
        .await
        .map_err(|x| {
            trace!("{x:?}");
            AuthCallbackError::SessionLoadingFailed
        })?
        .ok_or(AuthCallbackError::EmptySession)?;

    // Collect CSRF token from session
    let csrf_token = session
        .get::<CsrfToken>(CSRF_TOKEN)
        .ok_or(AuthCallbackError::CsrfTokenDeserializationError)?;

    // Destroy session, used for CSRF token storage
    state
        .session_store
        .destroy_session(session)
        .await
        .or(Err(AuthCallbackError::CsrfTokenSessionDestructionError))?;

    // Compare CSRF token from session and OAuth provider request
    if *csrf_token.secret() != query.state {
        return Err(AuthCallbackError::CsrfTokensMismatch)?;
    }

    // Exchange authentication code to token
    let client = Client::new();
    let token = state
        .oauth
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(&client)
        .await
        .or(Err(AuthCallbackError::CodeExchangeError))?
        .access_token()
        .to_owned();

    // Collect user info from OAuth provider
    let user_info_url = "https://api.github.com/user";
    let user_info = client
        .get(user_info_url)
        .bearer_auth(token.secret())
        .header(USER_AGENT, env!("CARGO_PKG_NAME"))
        .send()
        .await
        .or(Err(AuthCallbackError::UserInfoRequestError))?
        .error_for_status()
        .or(Err(AuthCallbackError::UserInfoRequestError))?
        .json::<UserInfo>()
        .await
        .or(Err(AuthCallbackError::UserInfoDeserializeResponseError))?;

    // Create session for user info storage
    let mut session = Session::new();
    session
        .insert(USER_INFO, &user_info)
        .or(Err(AuthCallbackError::UserInfoSerializationError))?;

    // Store session with user info and collect session id
    let session_id = state
        .session_store
        .store_session(session)
        .await
        .or(Err(AuthCallbackError::UserInfoStorageError))?
        .ok_or(AuthCallbackError::UserInfoStorageEmptyCookie)?;

    // Create cookie with user info session id
    let cookie = Cookie::build((COOKIE_NAME, session_id))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .path("/")
        .build();

    Ok((StatusCode::OK, cookie_jar.add(cookie), Json(user_info)))
}

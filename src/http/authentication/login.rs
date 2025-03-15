use async_session::{Session, SessionStore};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use oauth2::{CsrfToken, Scope};
use thiserror::Error;
use tracing::error;

use super::{COOKIE_NAME, CSRF_TOKEN};
use crate::http::{common::AppError, AppState};

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
enum LoginError {
    #[error("Cannot serialize CSRF Token")]
    CsrfTokenSerialization,
    #[error("Cannot store CSRF Token in session storage")]
    CsrfTokenStorage,
    #[error("CSRF Token was put in session storage, but returned empty cookie string")]
    CsrfTokenStorageEmptyCookie,
}

impl From<LoginError> for AppError {
    fn from(value: LoginError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            LoginError::CsrfTokenSerialization
            | LoginError::CsrfTokenStorageEmptyCookie
            | LoginError::CsrfTokenStorage => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

pub async fn login<T>(
    cookie_jar: CookieJar,
    State(state): State<AppState<T>>,
) -> Result<impl IntoResponse, AppError>
where
    T: SessionStore,
{
    // Construct authentication URL and CSRF token
    let (auth_uri, csrf_token) = state
        .oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user".to_owned()))
        .url();

    // Create session for CSRF token storage
    let mut session = Session::new();
    session
        .insert(CSRF_TOKEN, &csrf_token)
        .or(Err(LoginError::CsrfTokenSerialization))?;

    // Put session to session store and collect session ID
    let session_id = state
        .session_store
        .store_session(session)
        .await
        .or(Err(LoginError::CsrfTokenStorage))?
        .ok_or(LoginError::CsrfTokenStorageEmptyCookie)?;

    // Create cookie for session ID
    let cookie = Cookie::build((COOKIE_NAME, session_id))
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .path("/")
        .build();

    Ok((
        cookie_jar.add(cookie),
        Redirect::temporary(auth_uri.as_str()),
    ))
}

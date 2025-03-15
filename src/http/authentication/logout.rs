use async_session::SessionStore;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use thiserror::Error;
use tracing::error;

use super::COOKIE_NAME;
use crate::http::{common::AppError, AppState};

const HOME_PATH: &str = "/";

struct HomeRedirect;

impl IntoResponse for HomeRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary(HOME_PATH).into_response()
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LogoutError {
    #[error("Something went wrong")]
    UnexpectedError,
}

impl From<LogoutError> for AppError {
    fn from(value: LogoutError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            LogoutError::UnexpectedError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

pub async fn logout<T>(
    State(state): State<AppState<T>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, AppError>
where
    T: SessionStore,
{
    let Some(cookie) = cookie_jar.get(COOKIE_NAME) else {
        // No cookie set, we don't know whom to logout
        return Ok(HomeRedirect);
    };

    let Some(session) = state
        .session_store
        .load_session(cookie.to_string())
        .await
        // Session storage communication error, failig
        .or(Err(LogoutError::UnexpectedError))?
    else {
        // No session stored => already logged out
        return Ok(HomeRedirect);
    };

    state
        .session_store
        .destroy_session(session)
        .await
        // Cannot destroy session, failing
        .or(Err(LogoutError::UnexpectedError))?;

    // Successfully logged out
    Ok(HomeRedirect)
}

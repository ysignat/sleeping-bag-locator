use reqwest::StatusCode;
use thiserror::Error;
use tracing::error;

use crate::http::common::AppError;

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum AuthCallbackError {
    #[error("No such cookie {cookie_name:?} in a jar")]
    NoSuchCookie { cookie_name: String },
    #[error("Error while fetching session from storage. This error was a direct following of: {internal}")]
    SessionLoadingFailed { internal: String },
    #[error("It was expected, that cookie in a jar will be stored as a `key=value`")]
    InappropriateCookieFormat,
    #[error("Session collected, but it's empty")]
    EmptySession,
    #[error("Cannot deserialize CSRF token")]
    CsrfTokenDeserializationError,
    #[error(
        "Cannot destroy session with CSRF token. This error was a direct following of: {internal}"
    )]
    CsrfTokenSessionDestructionError { internal: String },
    #[error("CSRF token from session diverges from one from OAuth provider")]
    CsrfTokensMismatch,
    #[error("Cannot exchange code for token. This error was a direct following of: {internal}")]
    CodeExchangeError { internal: String },
    #[error("Error while sending request for User Info. This error was a direct following of: {internal}")]
    UserInfoRequestError { internal: String },
    #[error(
        "Cannot deserialize User Info response. This error was a direct following of: {internal}"
    )]
    UserInfoDeserializeResponseError { internal: String },
    #[error("Cannot serialize User Info. This error was a direct following of: {internal}")]
    UserInfoSerializationError { internal: String },
    #[error("Cannot store User Info in session storage. This error was a direct following of: {internal}")]
    UserInfoStorageError { internal: String },
    #[error("User Info was put in session storage, but returned empty cookie string")]
    UserInfoStorageEmptyCookie,
}

impl From<AuthCallbackError> for AppError {
    fn from(value: AuthCallbackError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            AuthCallbackError::NoSuchCookie { cookie_name: _ }
            | AuthCallbackError::InappropriateCookieFormat
            | AuthCallbackError::SessionLoadingFailed { internal: _ }
            | AuthCallbackError::EmptySession
            | AuthCallbackError::CsrfTokenDeserializationError
            | AuthCallbackError::CsrfTokenSessionDestructionError { internal: _ }
            | AuthCallbackError::CsrfTokensMismatch
            | AuthCallbackError::CodeExchangeError { internal: _ }
            | AuthCallbackError::UserInfoRequestError { internal: _ }
            | AuthCallbackError::UserInfoDeserializeResponseError { internal: _ }
            | AuthCallbackError::UserInfoSerializationError { internal: _ }
            | AuthCallbackError::UserInfoStorageError { internal: _ }
            | AuthCallbackError::UserInfoStorageEmptyCookie => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LoginError {
    #[error("Cannot serialize CSRF Token. This error was a direct following of: {internal}")]
    CsrfTokenSerialization { internal: String },
    #[error("Cannot store CSRF Token in session storage. This error was a direct following of: {internal}")]
    CsrfTokenStorage { internal: String },
    #[error("CSRF Token was put in session storage, but returned empty cookie string")]
    CsrfTokenStorageEmptyCookie,
}

impl From<LoginError> for AppError {
    fn from(value: LoginError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            LoginError::CsrfTokenSerialization { internal: _ }
            | LoginError::CsrfTokenStorageEmptyCookie
            | LoginError::CsrfTokenStorage { internal: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

#[derive(Error, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LogoutError {
    #[error("Cannot load session from storage. This error was a direct following of: {internal}")]
    SessionLoadError { internal: String },
    #[error("Cannot destroy session in storage. This error was a direct following of: {internal}")]
    SessionDestructionError { internal: String },
}

impl From<LogoutError> for AppError {
    fn from(value: LogoutError) -> Self {
        error!("{:#?}", value.to_string());
        let status_code = match value {
            LogoutError::SessionLoadError { internal: _ }
            | LogoutError::SessionDestructionError { internal: _ } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        Self {
            status_code,
            details: String::new(),
        }
    }
}

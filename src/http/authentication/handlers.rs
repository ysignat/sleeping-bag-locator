use async_session::Session;
use axum::{
    debug_handler,
    extract::{Query, State},
    http::{header::USER_AGENT, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use reqwest::Client;

use super::{
    dtos::AuthCallbackQuery,
    errors::{AuthCallbackError, LoginError, LogoutError},
    HomeRedirect,
    UserInfo,
    COOKIE_NAME,
    CSRF_TOKEN,
    USER_INFO,
};
use crate::http::{common::AppError, AppState};

#[debug_handler]
pub async fn auth_callback(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
    Query(query): Query<AuthCallbackQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Get session ID cookie
    let cookie = cookie_jar
        .get(COOKIE_NAME)
        .ok_or(AuthCallbackError::NoSuchCookie {
            cookie_name: COOKIE_NAME.to_owned(),
        })?
        .to_string()
        .split_once('=')
        .ok_or(AuthCallbackError::InappropriateCookieFormat)?
        .1
        .to_owned();

    // Collect session from session store
    let session = state
        .session_store
        .load_session(cookie)
        .await
        .map_err(|x| AuthCallbackError::SessionLoadingFailed {
            internal: x.to_string(),
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
        .map_err(|x| AuthCallbackError::CsrfTokenSessionDestructionError {
            internal: x.to_string(),
        })?;

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
        .map_err(|x| AuthCallbackError::CodeExchangeError {
            internal: x.to_string(),
        })?
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
        .map_err(|x| AuthCallbackError::UserInfoRequestError {
            internal: x.to_string(),
        })?
        .error_for_status()
        .map_err(|x| AuthCallbackError::UserInfoRequestError {
            internal: x.to_string(),
        })?
        .json::<UserInfo>()
        .await
        .map_err(|x| AuthCallbackError::UserInfoDeserializeResponseError {
            internal: x.to_string(),
        })?;

    // Create session for user info storage
    let mut session = Session::new();
    session.insert(USER_INFO, &user_info).map_err(|x| {
        AuthCallbackError::UserInfoSerializationError {
            internal: x.to_string(),
        }
    })?;

    // Store session with user info and collect session id
    let session_id = state
        .session_store
        .store_session(session)
        .await
        .map_err(|x| AuthCallbackError::UserInfoStorageError {
            internal: x.to_string(),
        })?
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

#[debug_handler]
pub async fn login(
    cookie_jar: CookieJar,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
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
        .map_err(|x| LoginError::CsrfTokenSerialization {
            internal: x.to_string(),
        })?;

    // Put session to session store and collect session ID
    let session_id = state
        .session_store
        .store_session(session)
        .await
        .map_err(|x| LoginError::CsrfTokenStorage {
            internal: x.to_string(),
        })?
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

#[debug_handler]
pub async fn logout(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let Some(cookie) = cookie_jar.get(COOKIE_NAME) else {
        // No cookie set, we don't know whom to logout
        return Ok(HomeRedirect);
    };

    let Some(session) = state
        .session_store
        .load_session(cookie.to_string())
        .await
        // Session storage communication error, failig
        .map_err(|x| LogoutError::SessionLoadError {
            internal: x.to_string(),
        })?
    else {
        // No session stored => already logged out
        return Ok(HomeRedirect);
    };

    state
        .session_store
        .destroy_session(session)
        .await
        // Cannot destroy session, failing
        .map_err(|x| LogoutError::SessionDestructionError {
            internal: x.to_string(),
        })?;

    // Successfully logged out
    Ok(HomeRedirect)
}

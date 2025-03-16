use async_session::{MemoryStore, SessionStore};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Redirect, Response},
    RequestPartsExt,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use super::{COOKIE_NAME, USER_INFO};

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
impl<S> FromRequestParts<S> for UserInfo
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

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

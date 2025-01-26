use async_session::MemoryStore;
use axum::extract::FromRef;
pub use callback::auth_callback;
pub use login::login;
pub use logout::logout;

use crate::http::AppState;

mod callback;
mod login;
mod logout;
mod user_info;

const CSRF_TOKEN: &str = "csrf_token";
const USER_INFO: &str = "user_info";
const COOKIE_NAME: &str = "session";

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}

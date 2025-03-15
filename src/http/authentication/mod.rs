pub use callback::auth_callback;
pub use login::login;
pub use logout::logout;

mod callback;
mod login;
mod logout;
mod user_info;

const CSRF_TOKEN: &str = "csrf_token";
const USER_INFO: &str = "user_info";
const COOKIE_NAME: &str = "session";

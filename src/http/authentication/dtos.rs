use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    pub code: String,
    pub state: String,
}

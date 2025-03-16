use std::sync::Arc;

use async_session::SessionStore;
pub use authentication::{auth_callback, login, logout};
pub use items::{create_item, delete_item, get_item, health, list_items, update_item};
use oauth2::{
    basic::{BasicErrorResponseType, BasicTokenType},
    Client,
    EmptyExtraTokenFields,
    EndpointNotSet,
    EndpointSet,
    RevocationErrorResponseType,
    StandardErrorResponse,
    StandardRevocableToken,
    StandardTokenIntrospectionResponse,
    StandardTokenResponse,
};

use crate::dao::ItemsDao;

mod authentication;
mod common;
mod items;

type OauthClient = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<dyn ItemsDao + Send + Sync>,
    pub session_store: Arc<dyn SessionStore + Send + Sync>,
    pub oauth: OauthClient,
}

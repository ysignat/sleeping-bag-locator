use std::sync::Arc;

use async_session::SessionStore;
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

use crate::dao::{ItemsDao, UsersDao};

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
    pub users: Arc<dyn UsersDao + Send + Sync>,
    pub session_store: Arc<dyn SessionStore + Send + Sync>,
    pub oauth: OauthClient,
}

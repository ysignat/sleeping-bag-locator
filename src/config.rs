#![allow(clippy::struct_field_names)]
use std::net::{IpAddr, Ipv4Addr};

use clap::{value_parser, Args, Parser, ValueEnum};
use tracing::Level;

#[derive(Parser, Debug)]
#[clap(version, about)]
pub struct Config {
    #[command(flatten)]
    pub runtime: Runtime,
    #[command(flatten)]
    pub logging: Logging,
    #[command(flatten)]
    pub authentication: Authentication,
    #[command(flatten)]
    pub session_store: SessionStore,
}

#[derive(Args, Clone, Debug)]
pub struct Runtime {
    #[arg(long, env = "HOST", default_value = Ipv4Addr::LOCALHOST.to_string())]
    pub bind_host: IpAddr,
    #[arg(long, env = "PORT", value_parser = value_parser!(u16).range(1..), default_value = "8080")]
    pub bind_port: u16,
    #[arg(long, env, default_value_t, value_enum)]
    pub dao_type: DaoType,
}

#[derive(Args, Clone, Debug)]
pub struct Logging {
    #[arg(long, env, default_value = "INFO")]
    pub log_level: Level,
    #[arg(long, env, default_value_t, value_enum)]
    pub log_format: LogFormat,
}

#[derive(Clone, ValueEnum, Default, Debug)]
pub enum LogFormat {
    Json,
    #[default]
    Default,
    Pretty,
}

#[derive(Clone, ValueEnum, Default, Debug)]
pub enum DaoType {
    Mocked,
    #[default]
    HashMap,
}

#[derive(Args, Clone, Debug)]
pub struct Authentication {
    #[arg(long, env, default_value = "")]
    pub oauth_client_id: String,
    #[arg(long, env, default_value = "")]
    pub oauth_client_secret: String,
}

#[derive(Clone, ValueEnum, Default, Debug)]
pub enum SessionStoreType {
    Memory,
    #[default]
    Redis,
}

#[derive(Args, Clone, Debug)]
pub struct SessionStore {
    #[arg(long, env, default_value_t, value_enum)]
    pub session_store_type: SessionStoreType,
    #[arg(long, env, default_value = "")]
    pub session_store_dsn: String,
}

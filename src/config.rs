use std::net::{IpAddr, Ipv4Addr};

use clap::{value_parser, Args, Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(version, about, ignore_errors = true)] // we need to ignore errors here cause somehow YCloud runs serverless container with ["/bin/sh", "/bin/api"] CLI args and it ruins clap
pub struct Config {
    #[command(flatten)]
    pub runtime_args: AppArgs,
}

#[derive(Args, Clone, Debug)]
pub struct AppArgs {
    #[arg(long, env = "HOST", default_value = Ipv4Addr::LOCALHOST.to_string())]
    pub bind_host: IpAddr,
    #[arg(long, env = "PORT", value_parser = value_parser!(u16).range(1..), default_value = "8080")]
    pub bind_port: u16,
    #[arg(long, env, default_value_t, value_enum)]
    pub dao_type: DaoType,
}

#[derive(Clone, ValueEnum, Default, Debug)]
pub enum DaoType {
    Mocked,
    #[default]
    HashMap,
}

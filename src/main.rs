#![allow(clippy::module_name_repetitions)]
use clap::Parser;
use config::ServerConfig;
use home_dir::HomeDirExt;
use log::{debug, info, warn};
use std::{
    error::Error,
    net::{SocketAddrV4, TcpListener},
    path::PathBuf,
    sync::Arc,
};

mod cli;
mod config;
mod handles;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::Cli::parse();

    #[cfg(debug_assertions)]
    simple_logger::init_with_level(log::Level::Debug)?;
    #[cfg(not(debug_assertions))]
    simple_logger::init_with_level(log::Level::Info)?;

    info!("LeafHttp v{}", env!("CARGO_PKG_VERSION"));

    let config_path = args
        .config
        .unwrap_or(PathBuf::from("~/.config/leafhttp.yml").expand_home()?);
    debug!("Loading configuration from {}", config_path.display());

    let config = ServerConfig::load(&config_path)?;
    debug!("Configuration loaded");

    let server_bind_addr = SocketAddrV4::new(
        args.addr.unwrap_or(config.address),
        args.port.unwrap_or(config.port),
    );

    if server_bind_addr.port() == 80 {
        warn!("Binding to port 80 requires root permissions");
    }

    let server = TcpListener::bind(server_bind_addr)?;
    info!("Started listening on http://{server_bind_addr}/");

    #[allow(clippy::needless_pass_by_value)]
    handles::server_handle(server, Arc::new(config))
}

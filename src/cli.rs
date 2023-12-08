use clap::Parser;
use std::{net::Ipv4Addr, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Specify a custom path to the server configuration
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(long, value_name = "ADDR")]
    pub addr: Option<Ipv4Addr>,

    #[arg(long, value_name = "PORT")]
    pub port: Option<u16>,
}

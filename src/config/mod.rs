mod impls;

use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: Ipv4Addr,
    pub port: u16,
    #[serde(rename = "www-dir")]
    pub root: PathBuf,
    #[serde(rename = "index-file")]
    pub index_file: PathBuf,
    #[serde(rename = "path-traversal")]
    pub path_traversal: bool,
    #[serde(rename = "path-traversal-action")]
    pub pa_action: PathTravelsalAction,
    #[serde(rename = "max-connections")]
    pub max_conns: usize,
    #[serde(rename = "max-request-size")]
    pub req_size_limit: usize,
    #[serde(rename = "allow-http-versions")]
    pub allowed_http_vers: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub enum PathTravelsalAction {
    /// Immediately close the socket connection
    #[serde(rename = "KICK")]
    Kick,
    /// Return a 404 Not Found error
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    /// Return a 403 Forbidden error
    #[serde(rename = "FORBIDDEN")]
    Forbidden,
}

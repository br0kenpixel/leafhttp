use super::{PathTravelsalAction, ServerConfig};
use std::{
    io,
    net::Ipv4Addr,
    path::{Path, PathBuf},
};

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            address: Ipv4Addr::new(127, 0, 0, 1),
            port: 80,
            root: PathBuf::from("www"),
            index_file: PathBuf::from("index.html"),
            path_traversal: false,
            pa_action: PathTravelsalAction::Kick,
            max_conns: 4,
            req_size_limit: 10485760,
            allowed_http_vers: vec![1, 2],
        }
    }
}

impl ServerConfig {
    pub fn load(from: &Path) -> io::Result<Self> {
        confy::load_path(from).map_err(|why| io::Error::other(why.to_string()))
    }
}

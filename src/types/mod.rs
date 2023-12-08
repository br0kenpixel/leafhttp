use std::{
    collections::BTreeMap,
    io::{BufReader, BufWriter},
    net::TcpStream,
};

pub mod error;
pub mod method;
pub mod request;
pub mod response;
pub mod response_code;
pub mod version;

pub type Headers = BTreeMap<String, String>;
pub type BufClientRead = BufReader<TcpStream>;
pub type BufClientWrite = BufWriter<TcpStream>;
pub type VersionInt = u8;

use super::{error::Error, method::Method, Headers};
use crate::{config::ServerConfig, types::version::Version};
use std::{
    collections::BTreeMap,
    io::{BufRead, Cursor, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub http_ver: Version,
    pub method: Method,
    pub headers: Headers,
    pub resource: PathBuf,
    pub body: String,
}

impl HttpRequest {
    pub fn parse(req: String, config: Arc<ServerConfig>) -> Result<Self, Error> {
        let mut stream = Cursor::new(req);
        let mut buf = String::new();
        let mut headers = BTreeMap::new();

        if stream.read_line(&mut buf)? == 0 {
            return Err(Error::EmptyStream);
        }

        let (method, resource, http_ver) = Self::parse_init(&buf, &config.index_file)?;

        for line in stream.by_ref().lines() {
            let line = line?;

            if line.is_empty() {
                break;
            }

            let (header, value) = Self::parse_header(&line)?;
            headers.insert(header, value);
        }

        buf.clear();
        stream.read_to_string(&mut buf)?;

        Ok(Self {
            http_ver,
            method,
            headers,
            resource,
            body: buf,
        })
    }

    fn parse_init(line: &str, default_index: &Path) -> Result<(Method, PathBuf, Version), Error> {
        let line = line
            .strip_suffix("\r\n")
            .ok_or(Error::syntax("Missing \\r\\n in init header"))?;
        let mut parts = line.split(' ');

        let maybe_method = parts.next().ok_or(Error::syntax("Missing method"))?;
        let mut maybe_resource = parts.next().ok_or(Error::syntax("Missing resource"))?;
        let maybe_ver = parts.next().ok_or(Error::syntax("Missing HTTP version"))?;

        if parts.next().is_some() {
            return Err(Error::syntax("Extra data left in init header"));
        }

        if maybe_resource.starts_with('/') {
            maybe_resource = &maybe_resource[1..];
        }

        let method = maybe_method.try_into()?;
        let mut resource = PathBuf::from(maybe_resource);
        let version = maybe_ver.try_into()?;

        if resource.as_os_str().is_empty() {
            resource = default_index.to_path_buf();
        }

        Ok((method, resource, version))
    }

    fn parse_header(line: &str) -> Result<(String, String), Error> {
        let (name, value) = line
            .split_once(": ")
            .ok_or(Error::syntax("Can't parse header"))?;

        Ok((name.to_string(), value.to_string()))
    }
}

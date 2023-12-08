use crate::{
    config::ServerConfig,
    types::{
        method::Method, request::HttpRequest, response::HttpResponse, BufClientRead,
        BufClientWrite, Headers,
    },
};
use log::error;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    sync::Arc,
};

pub fn handle_client(
    mut client_reader: BufClientRead,
    mut client_writer: BufClientWrite,
    config: Arc<ServerConfig>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; config.req_size_limit];
    let req_size = client_reader.read(&mut buffer)?;
    buffer.truncate(req_size);

    let stringified = String::from_utf8(buffer).map_err(|_| "Request content is not UTF-8")?;
    let request = HttpRequest::parse(stringified, config.clone())?;
    let response = process_request(&request, config)?;

    let raw = response.finalize();
    client_writer.write_all(raw.as_bytes())?;
    client_writer.flush()?;

    Ok(())
}

fn process_request(
    req: &HttpRequest,
    config: Arc<ServerConfig>,
) -> Result<HttpResponse, Box<dyn Error>> {
    if !config
        .allowed_http_vers
        .iter()
        .any(|ver| *ver == req.http_ver.major())
    {
        error!(
            "Unsupported HTTP protocol version ({}), dropping",
            req.http_ver
        );

        return Err("Unsupported HTTP version".into());
    }

    if req.method != Method::Get {
        error!("Unsupported HTTP request");
        return Ok(HttpResponse::NOT_IMPLEMENTED);
    }

    let resource_path = {
        let mut res = config.root.clone();
        res.push(&req.resource);

        if !config.path_traversal {
            res = fs::canonicalize(res)?;
        }

        res
    };

    let content_type = mime_guess::from_path(&resource_path).first_or_text_plain();
    let content = fs::read_to_string(&resource_path);

    match content {
        Ok(content) => Ok(HttpResponse::new_with_content(
            req.http_ver,
            Headers::new(),
            content_type.essence_str(),
            content,
        )),
        Err(e) => Ok(e.into()),
    }
}

use super::{response_code::HttpStatus, version::Version, Headers};
use std::{
    collections::btree_map::Entry,
    io::{self, ErrorKind},
};

macro_rules! const_def {
    ($status: expr) => {
        Self::new(
            Self::PREDEF_VER,
            $status,
            Self::EMPTY_HEADER_MAP,
            String::new(),
        )
    };
}

pub struct HttpResponse {
    pub http_ver: Version,
    pub status: HttpStatus,
    pub headers: Headers,
    pub body: String,
}

impl HttpResponse {
    const PREDEF_VER: Version = Version::new(1, 1);
    const EMPTY_HEADER_MAP: Headers = Headers::new();

    pub const NOT_FOUND: Self = const_def!(HttpStatus::NotFound);
    pub const INTERNAL_ERROR: Self = const_def!(HttpStatus::InternalServerError);
    pub const NOT_IMPLEMENTED: Self = const_def!(HttpStatus::NotImplemented);
    pub const FORBIDDEN: Self = const_def!(HttpStatus::Forbidden);

    pub const fn new(
        http_ver: Version,
        status: HttpStatus,
        headers: Headers,
        body: String,
    ) -> Self {
        Self {
            http_ver,
            status,
            headers,
            body,
        }
    }

    pub fn new_with_content(
        http_ver: Version,
        mut headers: Headers,
        mime: &str,
        body: String,
    ) -> Self {
        headers.insert("Content-Type".to_string(), mime.to_string());
        headers.insert("Content-Length".to_string(), body.len().to_string());

        Self::new(http_ver, HttpStatus::Ok, headers, body)
    }

    pub fn finalize(mut self) -> String {
        let mut buf = String::new();

        self.ensure_headers();

        buf.push_str(&format!(
            "HTTP/{} {}\r\n",
            self.http_ver,
            self.status.to_string()
        ));

        for (header, value) in self.headers {
            buf.push_str(&format!("{header}: {value}\r\n"));
        }

        buf.push_str("\r\n");
        buf.push_str(&self.body);

        buf
    }

    fn ensure_headers(&mut self) {
        if let Entry::Vacant(entry) = self.headers.entry("Content-Type".to_string()) {
            entry.insert("text/html; charset=UTF-8".to_string());
        }

        if let Entry::Vacant(entry) = self.headers.entry("Content-Length".to_string()) {
            entry.insert(self.body.len().to_string());
        }
    }
}

impl From<io::Error> for HttpResponse {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => Self::NOT_FOUND,
            ErrorKind::PermissionDenied => Self::FORBIDDEN,
            /* IsADirectory */
            _ if value.raw_os_error().unwrap() == 21 => Self::NOT_FOUND,
            _ => Self::INTERNAL_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HttpResponse;
    use crate::types::{response_code::HttpStatus, version::Version, Headers};
    use maplit::btreemap;

    #[test]
    fn response_1() {
        let response = HttpResponse::new(
            Version::new(1, 1),
            HttpStatus::Ok,
            btreemap! {
                "Some-Random-Header".to_string() => "Some-Random-Value".to_string()
            },
            String::new(),
        );
        let raw = response.finalize();

        assert_eq!(
            raw,
            concat!(
                "HTTP/1.1 200 OK\r\n",
                "Content-Length: 0\r\n",
                "Content-Type: text/html; charset=UTF-8\r\n",
                "Some-Random-Header: Some-Random-Value\r\n",
                "\r\n"
            )
        );
    }

    #[test]
    fn response_2() {
        let response = HttpResponse::new(
            Version::new(2, 0),
            HttpStatus::Ok,
            btreemap! {
                "Some-Random-Header".to_string() => "Some-Random-Value".to_string(),
                "Xyz".to_string() => "ABC".to_string(),
            },
            String::from("Hello, World!\n"),
        );
        let raw = response.finalize();

        assert_eq!(
            raw,
            concat!(
                "HTTP/2.0 200 OK\r\n",
                "Content-Length: 14\r\n",
                "Content-Type: text/html; charset=UTF-8\r\n",
                "Some-Random-Header: Some-Random-Value\r\n",
                "Xyz: ABC\r\n",
                "\r\n",
                "Hello, World!\n"
            )
        );
    }

    #[test]
    fn response_3() {
        let response = HttpResponse::new(
            Version::new(1, 0),
            HttpStatus::InternalServerError,
            Headers::new(),
            String::new(),
        );
        let raw = response.finalize();

        assert_eq!(
            raw,
            concat!(
                "HTTP/1.0 500 Internal Server Error\r\n",
                "Content-Length: 0\r\n",
                "Content-Type: text/html; charset=UTF-8\r\n",
                "\r\n"
            )
        );
    }
}

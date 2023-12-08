#[derive(Clone, Copy)]
pub enum HttpStatus {
    Ok,
    NotFound,
    NotImplemented,
    InternalServerError,
    Forbidden,
}

impl HttpStatus {
    pub const fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
            Self::NotImplemented => 501,
            Self::InternalServerError => 500,
            Self::Forbidden => 401,
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::NotImplemented => "Not Implemented",
            Self::InternalServerError => "Internal Server Error",
            Self::Forbidden => "Forbidden",
        }
    }
}

impl ToString for HttpStatus {
    fn to_string(&self) -> String {
        format!("{} {}", self.code(), self.description())
    }
}

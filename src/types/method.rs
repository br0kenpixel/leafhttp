#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
}

impl TryFrom<&str> for Method {
    type Error = super::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "OPTIONS" => Ok(Self::Options),
            _ => Err(super::error::Error::UnknownMethod),
        }
    }
}

use super::{error::Error, VersionInt};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Version(VersionInt, VersionInt);

impl Version {
    pub const fn new(major: VersionInt, minor: VersionInt) -> Self {
        Self(major, minor)
    }

    pub const fn major(self) -> VersionInt {
        self.0
    }

    #[allow(unused)]
    pub const fn minor(self) -> VersionInt {
        self.1
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.0, self.1)
    }
}

impl TryFrom<&str> for Version {
    type Error = super::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.get(5..).ok_or(Error::InvalidHttpVersion)?;
        let (maybe_major, maybe_minor) = value.split_once('.').ok_or(Error::InvalidHttpVersion)?;

        let major = maybe_major
            .parse::<VersionInt>()
            .map_err(|_| Error::InvalidHttpVersion)?;
        let minor = maybe_minor
            .parse::<VersionInt>()
            .map_err(|_| Error::InvalidHttpVersion)?;

        Ok(Self(major, minor))
    }
}

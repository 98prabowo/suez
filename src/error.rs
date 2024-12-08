use std::{error, fmt::Display};

use crate::pool;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Pool(pool::Error),
    Io(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pool(error)   => write!(f, "{error}"),
            Self::Io(error)     => write!(f, "{error}"),
        }
    }
}

impl error::Error for Error {}

impl From<pool::Error> for Error {
    fn from(value: pool::Error) -> Self {
        Self::Pool(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

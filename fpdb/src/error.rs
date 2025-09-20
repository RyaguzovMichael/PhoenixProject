use std::convert::Infallible;

use uuid;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<Infallible> for Error {
    fn from(value: Infallible) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        Error {
            message: value.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error {
            message: String::from(value),
        }
    }
}

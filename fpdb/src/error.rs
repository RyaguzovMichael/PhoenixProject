use uuid;

pub struct Error {
    pub message: String,
}

impl From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
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

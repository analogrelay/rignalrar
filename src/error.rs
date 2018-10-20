#[derive(Debug)]
pub enum Error {
    HttpError(hyper::Error),
    IoError(String),
    DeserializationError(serde_json::error::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Error {
        Error::HttpError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IoError(format!("{}", e))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error {
        Error::DeserializationError(e)
    }
}
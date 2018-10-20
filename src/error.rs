#[derive(Debug)]
pub enum Error {
    HttpError(hyper::Error),
    IoError(String),
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
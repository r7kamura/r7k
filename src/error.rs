#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Request(actix_web::client::SendRequestError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<actix_web::client::SendRequestError> for Error {
    fn from(error: actix_web::client::SendRequestError) -> Self {
        Error::Request(error)
    }
}

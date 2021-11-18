#[derive(Debug)]
pub enum Error {
    ActixPayload(actix_web::error::PayloadError),
    ActixRequest(actix_web::client::SendRequestError),
    Fronma(fronma::error::Error),
    Io(std::io::Error),
    SerdeYaml(serde_yaml::Error),
    String(std::string::FromUtf8Error),
}

impl From<actix_web::error::PayloadError> for Error {
    fn from(error: actix_web::error::PayloadError) -> Self {
        Error::ActixPayload(error)
    }
}

impl From<actix_web::client::SendRequestError> for Error {
    fn from(error: actix_web::client::SendRequestError) -> Self {
        Error::ActixRequest(error)
    }
}

impl From<fronma::error::Error> for Error {
    fn from(error: fronma::error::Error) -> Self {
        Error::Fronma(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::SerdeYaml(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::String(error)
    }
}

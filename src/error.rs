use derive_more::Display;

#[derive(Display, Debug)]
pub enum Error {
    IOFailure(std::io::Error),
    CoffeeError(coffee::Error),
    SerdeError(serde_json::Error),
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(original: std::io::Error) -> Error {
        Error::IOFailure(original)
    }
}

impl From<coffee::Error> for Error {
    fn from(original: coffee::Error) -> Error {
        Error::CoffeeError(original)
    }
}

impl From<serde_json::Error> for Error {
    fn from(original: serde_json::Error) -> Error {
        Error::SerdeError(original)
    }
}

impl From<Error> for coffee::Error {
    fn from(original: Error) -> coffee::Error {
        coffee::Error::IO(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", original),
        ))
    }
}

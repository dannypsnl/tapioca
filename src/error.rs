use from_pest::{ConversionError, Void};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parser(ConversionError<Void>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::Parser(err) => write!(f, "Parser error: {:?}", err),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}
impl From<ConversionError<Void>> for Error {
    fn from(err: ConversionError<Void>) -> Error {
        Error::Parser(err)
    }
}

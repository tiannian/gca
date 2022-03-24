#[derive(Debug)]
pub enum Error {
    ErrPrefix,
    ErrHexError(hex::FromHexError),
    ParseIntError(core::num::ParseIntError),
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::ErrHexError(e)
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(e: core::num::ParseIntError) -> Self {
        Error::ParseIntError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

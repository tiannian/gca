#[derive(Debug)]
pub enum Error {
    ErrUtf8,
}

impl From<core::str::Utf8Error> for Error {
    fn from(_: core::str::Utf8Error) -> Self {
        Error::ErrUtf8
    }
}

pub type Result<T> = core::result::Result<T, Error>;

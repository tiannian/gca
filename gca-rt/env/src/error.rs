pub enum Error {
    ErrUtf8(core::str::Utf8Error),
}

impl From<core::str::Utf8Error> for Error {
    fn from(e: core::str::Utf8Error) -> Self {
        Error::ErrUtf8(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

use core::fmt::Debug;

pub enum Error {
    ErrPrefix(&'static str),
    ErrHexError(hex::FromHexError),
    ParseIntError(core::num::ParseIntError),

    // Got 0, Expect 1,
    BytesSizeError(usize, usize),

    InvaildOutputType(u8),

    NoColonFound,
}

impl Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ErrPrefix(prefix) => write!(f, "prefix must be {}", prefix),
            Self::ErrHexError(e) => write!(f, "{:?}", e),
            Self::ParseIntError(e) => write!(f, "{:?}", e),
            Self::BytesSizeError(a, b) => write!(f, "Bytes size error. Got {}, Expect {}", a, b),
            Self::InvaildOutputType(a) => write!(f, "Invaild output type: {}", a),
            Self::NoColonFound => write!(f, "No Colon Found"),
        }
    }
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

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "wasmi-backend")]
    ErrWasmiError(wasmi::Error),

    ErrIntConvert(std::num::TryFromIntError),

    ErrInputsCount,
    ErrMustBeOperationInput,
    ErrNoUnspentOutputPreLoad,
    ErrOnlyDataCanLoad,
    ErrWasmNoMemory,
    ErrWasmAllocError,
    ErrReturnCode,
    ErrNoOperation,
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error::ErrIntConvert(e)
    }
}

impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Self {
        Error::ErrWasmiError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

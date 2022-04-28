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

    ParityWasmError(parity_wasm::elements::Error),
    InjectError,

    BackendError(String),
}

impl From<parity_wasm::elements::Module> for Error {
    fn from(_: parity_wasm::elements::Module) -> Self {
        Error::InjectError
    }
}

impl From<parity_wasm::elements::Error> for Error {
    fn from(e: parity_wasm::elements::Error) -> Error {
        Error::ParityWasmError(e)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error::ErrIntConvert(e)
    }
}

#[cfg(feature = "wasmi-backend")]
impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Self {
        Error::ErrWasmiError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

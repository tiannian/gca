pub enum Error {
    InputTypeIndexError,
}

pub type Result<T> = core::result::Result<T, Error>;

use crate::{Module, Result};

pub struct WasmiModule {
    pub(crate) m: wasmi::Module,
}

impl Module for WasmiModule {
    fn load_bytes(bytes: impl AsRef<[u8]>) -> Result<Self> {
        let m = wasmi::Module::from_buffer(bytes)?;

        Ok(WasmiModule { m })
    }
}

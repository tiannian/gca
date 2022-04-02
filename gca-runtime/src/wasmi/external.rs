pub struct WasmiExternal {}

impl wasmi::Externals for WasmiExternal {
    fn invoke_index(
        &mut self,
        _index: usize,
        _args: wasmi::RuntimeArgs,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        Ok(None)
    }
}

use std::collections::BTreeMap;

use gca_core::{OutputCore, OutputData, OutputOperation};

use crate::{Backend, Error, Instance, Module, Result, Val};

#[derive(Default)]
pub struct Operator {
    pub operations: BTreeMap<OutputOperation, OutputCore>,
}

impl Operator {
    pub fn execute_operation<B: Backend>(
        &self,
        operation: OutputOperation,
        is_check: bool,
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        let core = self
            .operations
            .get(&operation)
            .ok_or(Error::ErrNoOperation)?;
        if let OutputData::Data(code) = &core.data {
            self.execute(code, is_check, backend)
        } else {
            Err(Error::ErrOnlyDataCanLoad)
        }
    }

    fn execute<B: Backend>(
        &self,
        code: &[u8],
        check: bool,
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &[])?;

        let entry = if check {
            "_gca_operation_check_entry"
        } else {
            "_gca_operation_execute_entry"
        };

        if let Some(Val::I32(ret_code)) = instance.call_func(entry, &[])? {
            Ok((ret_code, instance))
        } else {
            Err(Error::ErrReturnCode)
        }
    }
}

// TODO: add unit test here.
#[cfg(test)]
mod tests {}

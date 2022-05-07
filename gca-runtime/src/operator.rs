use std::collections::BTreeMap;

use gca_core::{OutputCore, OutputOperation, OutputData};

use crate::{Result, Backend, Module, Val, Instance, Error};

pub struct Operator {
    pub operations: BTreeMap<OutputOperation, OutputCore>,
}

impl Operator {
    pub fn execute_operation<B: Backend>(&self, operation: OutputOperation, backend: B) -> Result<(i32, B::Instance)> {
        let core = self.operations.get(&operation).ok_or(Error::ErrNoOperation)?;
        if let OutputData::Data(code) = &core.data {
            self.execute(&code, backend)
        } else {
            Err(Error::ErrOnlyDataCanLoad)
        }
    }

    fn execute<B: Backend>(&self, code: &[u8], backend: B) -> Result<(i32, B::Instance)> {
        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &[])?;

        if let Some(Val::I32(ret_code)) =
            instance.call_func("_gca_operation_entry", &[])?
        {
            Ok((ret_code, instance))
        } else {
            Err(Error::ErrReturnCode)
        }
    }
}


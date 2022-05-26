use std::fmt::Debug;

use crate::{FuncDefine, Host, Instance, Memory, Val, ValTy};

pub struct ReceiptData<M> {
    pub data: Vec<u8>,
    func_def: Vec<FuncDefine>,
    instance: Option<M>,
}

impl<M> Clone for ReceiptData<M> {
    fn clone(&self) -> Self {
        Self {
            func_def: self.func_def.clone(),
            instance: None,
            data: self.data.clone(),
        }
    }
}

impl<M> Default for ReceiptData<M> {
    fn default() -> Self {
        let f = FuncDefine {
            name: "_set_data",
            parmas: vec![ValTy::I32, ValTy::I32],
            ret: None,
        };

        let func_def = vec![f];

        Self {
            func_def,
            instance: None,
            data: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum ReceiptDataHostError {
    ArgumentsFormat,
    CalledName,
    NoInstance,
    NoMemory,
    // TODO: Detial here.
    NoMemoryReadError,
}

impl From<ReceiptDataHostError> for Box<dyn Debug + Send + Sync> {
    fn from(e: ReceiptDataHostError) -> Self {
        Box::new(e)
    }
}

impl<M: Instance + 'static> Host<M> for ReceiptData<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, _instance: M) {}

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn Debug + Send + Sync>> {
        if name != "_set_data" {
            return Err(ReceiptDataHostError::CalledName.into());
        }

        if args.len() != 1 {
            return Err(ReceiptDataHostError::ArgumentsFormat.into());
        }

        if let (Some(Val::I32(ptr)), Some(Val::I32(len))) = (args.get(0), args.get(1)) {
            let instance = self
                .instance
                .as_ref()
                .ok_or(ReceiptDataHostError::NoInstance)?;
            let memory = instance
                .get_memory("memory")
                .ok_or(ReceiptDataHostError::NoMemory)?;

            let mut data = vec![0u8; *len as usize];

            memory
                .read(*ptr as usize, &mut data)
                .map_err(|_| ReceiptDataHostError::NoMemoryReadError)?;
        } else {
            return Err(ReceiptDataHostError::ArgumentsFormat.into());
        }

        Ok(None)
    }
}

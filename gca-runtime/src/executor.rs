use std::{collections::BTreeMap, marker::PhantomData};

use gca_core::{Block, InputOperation, Output, OutputData, OutputId, OutputOperation, Transaction};

use crate::{Backend, BlockchainEnv, Error, Host, Instance, Memory, Module, Result, Val};

pub struct Executor<B, H> {
    transaction: Transaction,
    pub outputs: BTreeMap<OutputId, Output>,
    pub operations: BTreeMap<OutputOperation, OutputId>,
    marker_h: PhantomData<H>,
    marker_b: PhantomData<B>,
}

impl<B, H> Executor<B, H>
where
    B: Backend<H>,
    H: Host,
{
    // Create a new executor to execute transaction.
    pub fn new(transaction: Transaction) -> Self {
        Self {
            transaction,
            outputs: BTreeMap::new(),
            operations: BTreeMap::new(),
            marker_h: PhantomData,
            marker_b: PhantomData,
        }
    }

    /// Validate this transaction's all input is unlocked?.
    pub fn unlock_by_index(&self, idx: usize) -> Result<i32> {
        if let Some(input) = self.transaction.inputs.get(idx) {
            // try to get input's output.
            if !matches!(input.operation, InputOperation::Input(_)) {
                return Err(Error::ErrMustBeOperationInput);
            }

            if let Some(output) = self.outputs.get(&input.output_id) {
                // try to get lock code.
                if let Some(lock_output) = self.outputs.get(&output.locker) {
                    if let OutputData::Data(code) = &lock_output.data {
                        let data = &input.unlock;
                        Ok(self.unlock(code, data)?)
                    } else {
                        return Err(Error::ErrOnlyDataCanLoad);
                    }
                } else {
                    return Err(Error::ErrNoUnspentOutputPreLoad);
                }
            } else {
                return Err(Error::ErrNoUnspentOutputPreLoad);
            }
        } else {
            Err(Error::ErrInputsCount)
        }
    }

    fn unlock(&self, code: &[u8], data: &[u8]) -> Result<i32> {
        // build env and tx backend.
        let mut backend = B::new(&[]);

        let module = B::Module::load_bytes(code)?;

        let instance = backend.instance(&module, &[])?;

        if let Some(memory) = instance.get_memory("memory") {
            // alloc memory space.

            let len: i32 = data.len().try_into()?;

            if let Some(Val::I32(ptr)) = instance.call_func("_gca_env_alloc", &[Val::I32(len)])? {
                let offset: usize = ptr.try_into()?;

                memory.write(offset, data)?;

                // call entry.

                if let Some(Val::I32(ret_code)) =
                    instance.call_func("_gca_unlock_entry", &[Val::I32(ptr)])?
                {
                    Ok(ret_code)
                } else {
                    Err(Error::ErrReturnCode)
                }
            } else {
                Err(Error::ErrWasmAllocError)
            }
        } else {
            Err(Error::ErrWasmNoMemory)
        }
    }

    pub fn verify_operation(&self, operation: OutputOperation) -> Result<i32> {
        // get output.
        if let Some(output_id) = self.operations.get(&operation) {
            if let Some(output) = self.outputs.get(output_id) {
                if let OutputData::Data(code) = &output.data {
                    self.verify_operation_script(&code)
                } else {
                    return Err(Error::ErrOnlyDataCanLoad);
                }
            } else {
                Err(Error::ErrNoUnspentOutputPreLoad)
            }
        } else {
            Err(Error::ErrNoOperation)
        }
    }

    fn verify_operation_script(&self, code: &[u8]) -> Result<i32> {
        // TODO: build all host in backend.
        let mut backend = B::new(&[]);

        let module = B::Module::load_bytes(code)?;

        let instance = backend.instance(&module, &[])?;

        if let Some(Val::I32(i)) = instance.call_func("_gca_operation_entry", &[])? {
            Ok(i)
        } else {
            Err(Error::ErrReturnCode)
        }
    }

    pub fn verify_sub_transaction(&self, index: usize) -> Result<i32> {
        if let Some(output) = self.transaction.outputs.get(index) {

            Ok(0)

        } else {
            Err(Error::ErrNoUnspentOutputPreLoad)
        }
    }
}

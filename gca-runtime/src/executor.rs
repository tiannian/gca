use std::{collections::BTreeMap, marker::PhantomData};

use gca_core::{InputOperation, Output, OutputData, OutputId, OutputOperation, Transaction};

use crate::{Backend, Error, Host, Instance, Memory, Module, ModuleInfo, Result, Val};

pub struct Executor<B, H> {
    transaction: Transaction,
    pub outputs: BTreeMap<OutputId, Output>,
    pub operations: BTreeMap<OutputOperation, OutputId>,
    reference: BTreeMap<u32, Vec<(String, OutputId)>>,
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
        let mut reference: BTreeMap<u32, Vec<(String, OutputId)>> = BTreeMap::new();

        for input in &transaction.inputs {
            if let InputOperation::Reference(name, i) = &input.operation {
                if let Some(v) = reference.get_mut(i) {
                    v.push((name.clone(), input.output_id.clone()));
                } else {
                    let v = vec![(name.clone(), input.output_id.clone())];

                    reference.insert(*i, v);
                }
            }
        }

        Self {
            transaction,
            reference,
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
                        Err(Error::ErrOnlyDataCanLoad)
                    }
                } else {
                    Err(Error::ErrNoUnspentOutputPreLoad)
                }
            } else {
                Err(Error::ErrNoUnspentOutputPreLoad)
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

        let memory = instance
            .get_memory("memory")
            .ok_or(Error::ErrWasmNoMemory)?;
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
    }

    pub fn verify_operation(&self, operation: OutputOperation) -> Result<i32> {
        // get output.
        let output_id = self
            .operations
            .get(&operation)
            .ok_or(Error::ErrNoOperation)?;
        let output = self
            .outputs
            .get(output_id)
            .ok_or(Error::ErrNoUnspentOutputPreLoad)?;
        if let OutputData::Data(code) = &output.data {
            self.verify_operation_script(code)
        } else {
            Err(Error::ErrOnlyDataCanLoad)
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

    pub fn verify_output(&self, index: usize) -> Result<i32> {
        let output = self
            .transaction
            .outputs
            .get(index)
            .ok_or(Error::ErrNoUnspentOutputPreLoad)?;
        if let Some(verifier) = &output.verifier {
            let i = self
                .outputs
                .get(verifier)
                .ok_or(Error::ErrNoUnspentOutputPreLoad)?;
            if let OutputData::Data(code) = &i.data {
                self.verify_output_script(index, code)
            } else {
                Err(Error::ErrOnlyDataCanLoad)
            }
        } else {
            Ok(0)
        }
    }

    fn verify_output_script(&self, index: usize, code: &[u8]) -> Result<i32> {
        let mut deps = Vec::new();

        // Add all dependience.
        let mut backend = B::new(&[]);

        // Load dep module.
        if let Some(v) = self.reference.get(&index.try_into()?) {
            for (name, output_id) in v {
                let output = self
                    .outputs
                    .get(output_id)
                    .ok_or(Error::ErrNoUnspentOutputPreLoad)?;

                if let OutputData::Data(data) = &output.data {
                    let module = B::Module::load_bytes(data)?;

                    let module_info = ModuleInfo {
                        name,
                        module,
                    };

                    deps.push(module_info);
                } else {
                    return Err(Error::ErrOnlyDataCanLoad);
                }
            }
        }

        let module = B::Module::load_bytes(code)?;

        let instance = backend.instance(&module, &deps)?;

        // execute here.
        if let Some(Val::I32(ret_code)) = instance.call_func("_gca_verifier_entry", &[])? {
            Ok(ret_code)
        } else {
            Err(Error::ErrReturnCode)
        }
    }
}

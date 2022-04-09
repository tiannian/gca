use std::collections::BTreeMap;

use gca_core::{InputOperation, Output, OutputData, OutputId, OutputOperation, Transaction};

use crate::{Backend, Error, Instance, Memory, Module, ModuleInfo, Result, Val};

pub struct Executor {
    transaction: Transaction,
    pub outputs: BTreeMap<OutputId, Output>,
    pub operations: BTreeMap<OutputOperation, OutputId>,
    reference: BTreeMap<u32, Vec<(String, OutputId)>>,
}

impl Executor {
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
        }
    }

    /// Validate this transaction's all input is unlocked?.
    pub fn unlock_by_index<B: Backend>(
        &self,
        idx: usize,
        backend: B,
    ) -> Result<(i32, B::Instance)> {
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
                        Ok(self.unlock(code, data, backend)?)
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

    fn unlock<B: Backend>(
        &self,
        code: &[u8],
        data: &[u8],
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        // build env and tx backend.
        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &[])?;

        let memory = instance
            .get_memory("memory")
            .ok_or(Error::ErrWasmNoMemory)?;
        // alloc memory space.

        let len: i32 = data.len().try_into()?;

        let ptr = instance.call_func("_gca_env_alloc", &[Val::I32(len)])?;

        log::info!("alloced ptr: {:?}", ptr);

        if let Some(Val::I32(ptr)) = ptr {
            let offset: usize = ptr.try_into()?;

            memory.write(offset, data)?;

            // call entry.

            if let Some(Val::I32(ret_code)) =
                instance.call_func("_gca_unlock_entry", &[Val::I32(ptr)])?
            {
                Ok((ret_code, instance))
            } else {
                Err(Error::ErrReturnCode)
            }
        } else {
            Err(Error::ErrWasmAllocError)
        }
    }

    pub fn verify_operation<B: Backend>(
        &self,
        operation: OutputOperation,
        backend: B,
    ) -> Result<(i32, B::Instance)> {
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
            self.verify_operation_script(code, backend)
        } else {
            Err(Error::ErrOnlyDataCanLoad)
        }
    }

    fn verify_operation_script<B: Backend>(&self, code: &[u8], backend: B) -> Result<(i32, B::Instance)> {
        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &[])?;

        if let Some(Val::I32(i)) = instance.call_func("_gca_operation_entry", &[])? {
            Ok((i, instance))
        } else {
            Err(Error::ErrReturnCode)
        }
    }

    pub fn verify_output<B: Backend>(&self, index: usize, backend: B) -> Result<Option<(i32, B::Instance)>> {
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
                self.verify_output_script(index, code, backend).map(|v| Some(v))
            } else {
                Err(Error::ErrOnlyDataCanLoad)
            }
        } else {
            Ok(None)
        }
    }

    fn verify_output_script<B: Backend>(
        &self,
        index: usize,
        code: &[u8],
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        let mut deps = Vec::new();

        // Load dep module.
        if let Some(v) = self.reference.get(&index.try_into()?) {
            for (name, output_id) in v {
                let output = self
                    .outputs
                    .get(output_id)
                    .ok_or(Error::ErrNoUnspentOutputPreLoad)?;

                if let OutputData::Data(data) = &output.data {
                    let module = B::Module::load_bytes(data)?;

                    let module_info = ModuleInfo { name, module };

                    deps.push(module_info);
                } else {
                    return Err(Error::ErrOnlyDataCanLoad);
                }
            }
        }

        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &deps)?;

        // execute here.
        if let Some(Val::I32(ret_code)) = instance.call_func("_gca_verifier_entry", &[])? {
            Ok((ret_code, instance))
        } else {
            Err(Error::ErrReturnCode)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{env, fs, path::Path};

    use gca_core::{
        Amount, Input, InputOperation, Output, OutputData, OutputId, OutputOperation, Transaction,
    };

    use crate::{host, Backend, Executor};

    pub fn build_tx(wasm_output_id: &OutputId, unspend_output_id: &OutputId) -> Transaction {
        // Build tx.
        //
        // Inputs:
        // 1. Spent output
        //     - output_id: 0x00:100,
        //     - unlock: 0x,
        //     - operation: Input(0),
        //
        // Outputs:
        // 1. Transfer result
        //     - locker: 0x00:200,
        //     - data: Amount:99,
        //     - verifier: 0x00:200,
        let txhash = Default::default();

        let mut inputs = Vec::new();

        // Try to cost this output.
        let input1 = Input {
            output_id: unspend_output_id.clone(),
            unlock: Vec::new(),
            operation: InputOperation::Input(0),
        };

        inputs.push(input1);

        let mut outputs = Vec::new();

        let output = Output {
            data: OutputData::NativeToken(Amount(99)),
            locker: wasm_output_id.clone(),
            verifier: Some(wasm_output_id.clone()),
            operation: OutputOperation(2),
        };
        outputs.push(output);

        let output = Output {
            data: OutputData::NativeToken(Amount(1)),
            locker: wasm_output_id.clone(),
            verifier: Some(wasm_output_id.clone()),
            operation: OutputOperation(0),
        };
        outputs.push(output);

        let memos = Default::default();

        Transaction {
            txhash,
            inputs,
            outputs,
            memos,
        }
    }

    pub fn test_empty<B: Backend>() {
        // Read wasm
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/empty.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_exeutor(bin);

        let unlock_backend = B::new();
        let code = executor.unlock_by_index(0, unlock_backend).unwrap();
        assert_eq!(code.0, 0);

        let operation_backend = B::new();

        let operation = OutputOperation(0);
        let code = executor
            .verify_operation(operation, operation_backend)
            .unwrap();
        assert_eq!(code.0, 0);

        let verifier_backend = B::new();
        let code = executor.verify_output(0, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
        let verifier_backend = B::new();
        let code = executor.verify_output(1, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
    }

    pub fn test_log<B: Backend>() {
        // Read wasm
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/log.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_exeutor(bin);

        // instant host
        let log = host::Logger::<B::Instance>::new();

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());
        let code = executor.unlock_by_index(0, unlock_backend).unwrap();

        assert_eq!(code.0, 0);

        let mut operation_backend = B::new();
        operation_backend.add_host("_gca_log", log.clone());

        let operation = OutputOperation(0);
        let code = executor
            .verify_operation(operation, operation_backend)
            .unwrap();
        assert_eq!(code.0, 0);

        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        let code = executor.verify_output(0, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        let code = executor.verify_output(1, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
    }

    pub fn test_chain_id<B: Backend>() {
        // Read wasm
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/chain_id.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_exeutor(bin);

        // instant host
        let log = host::Logger::<B::Instance>::new();
        let env = host::Env::<B::Instance>::new("chain id");

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());
        unlock_backend.add_host("_gca_env", env.clone());
        let code = executor.unlock_by_index(0, unlock_backend).unwrap();
        assert_eq!(code.0, 0);

        let mut operation_backend = B::new();
        operation_backend.add_host("_gca_log", log.clone());
        operation_backend.add_host("_gca_env", env.clone());

        let operation = OutputOperation(0);
        let code = executor
            .verify_operation(operation, operation_backend)
            .unwrap();
        assert_eq!(code.0, 0);

        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        verifier_backend.add_host("_gca_env", env.clone());

        let code = executor.verify_output(0, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        verifier_backend.add_host("_gca_env", env.clone());

        let code = executor.verify_output(1, verifier_backend).unwrap().unwrap();
        assert_eq!(code.0, 0);
    }

    pub fn build_exeutor(bin: Vec<u8>) -> Executor {
        let wasm_output_id = OutputId::from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000001:0",
        )
        .unwrap();

        let unspend_output_id = OutputId::from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000002:0",
        )
        .unwrap();

        let tx = build_tx(&wasm_output_id, &unspend_output_id);

        let mut executor = Executor::new(tx);

        // Insert backend output.
        let unspend_output = Output {
            data: OutputData::NativeToken(Amount(100)),
            locker: wasm_output_id.clone(),
            verifier: Some(wasm_output_id.clone()),
            operation: OutputOperation(2),
        };
        executor.outputs.insert(unspend_output_id, unspend_output);

        let wasm_output = Output {
            data: OutputData::Data(bin),
            locker: wasm_output_id.clone(),
            verifier: Some(wasm_output_id.clone()),
            operation: OutputOperation(1),
        };
        executor.outputs.insert(wasm_output_id.clone(), wasm_output);

        let operation = OutputOperation(0);
        executor
            .operations
            .insert(operation.clone(), wasm_output_id.clone());

        executor
    }
}

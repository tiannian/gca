use std::collections::BTreeMap;

use gca_core::{Input, InputOperation, OutputCore, OutputData, OutputId};

use crate::{Backend, Error, Instance, Module, Result, Val};

#[derive(Default)]
pub struct Unlocker {
    pub cores: BTreeMap<OutputId, OutputCore>,
}

impl Unlocker {
    /// Validate this transaction's all input is unlocked?.
    pub fn unlock_input_by_index<B: Backend>(
        &self,
        idx: u32,
        input: &Input,
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        // try to get input's output.
        if !matches!(input.operation, InputOperation::Input(_)) {
            return Err(Error::ErrMustBeOperationInput);
        }

        let output = self
            .cores
            .get(&input.output_id)
            .ok_or(Error::ErrNoUnspentOutputPreLoad)?;
        let lock_output = self
            .cores
            .get(&output.locker)
            .ok_or(Error::ErrNoUnspentOutputPreLoad)?;

        if let OutputData::Data(code) = &lock_output.data {
            Ok(self.unlock(code, idx, backend)?)
        } else {
            Err(Error::ErrOnlyDataCanLoad)
        }
    }

    fn unlock<B: Backend>(&self, code: &[u8], idx: u32, backend: B) -> Result<(i32, B::Instance)> {
        // build env and tx backend.
        let module = B::Module::load_bytes(code)?;

        let mut instance = backend.instance(&module, &[])?;

        if let Some(Val::I32(ret_code)) =
            instance.call_func("_gca_unlock_entry", &[Val::I32(idx as i32)])?
        {
            Ok((ret_code, instance))
        } else {
            Err(Error::ErrReturnCode)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{env, fs, path::Path};

    use gca_core::{Amount, Input, InputOperation, OutputCore, OutputData, OutputId, Verifier};

    use crate::{host, Backend, Unlocker};

    pub fn build_input() -> Input {
        let unspend_output_id = OutputId::from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000002:0",
        )
        .unwrap();

        Input {
            output_id: unspend_output_id.clone(),
            unlock: Vec::new(),
            operation: InputOperation::Input(0),
        }
    }

    pub fn test_empty_unlocker<B: Backend>() {
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/empty.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_unlocker(bin);

        let unlock_backend = B::new();

        let input = build_input();

        let code = executor
            .unlock_input_by_index(0, &input, unlock_backend)
            .unwrap();

        assert_eq!(code.0, 0);
    }

    pub fn test_log_unlocker<B: Backend>() {
        // Read wasm
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/log.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_unlocker(bin);

        // instant host
        let log = host::Logger::<B::Instance>::default();

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());

        let input = build_input();
        let code = executor
            .unlock_input_by_index(0, &input, unlock_backend)
            .unwrap();

        assert_eq!(code.0, 0);
    }

    pub fn test_chain_id_unlocker<B: Backend>() {
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/chain_id.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let executor = build_unlocker(bin);

        // instant host
        let log = host::Logger::<B::Instance>::default();
        let env = host::Env::<B::Instance>::new("chain id");

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());
        unlock_backend.add_host("_gca_env", env.clone());

        let input = build_input();
        let code = executor
            .unlock_input_by_index(0, &input, unlock_backend)
            .unwrap();
        assert_eq!(code.0, 0);
    }

    pub fn build_unlocker(bin: Vec<u8>) -> Unlocker {
        let wasm_output_id = OutputId::from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000001:0",
        )
        .unwrap();

        let unspend_output_id = OutputId::from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000002:0",
        )
        .unwrap();

        let mut unlocker = Unlocker::default();

        let verifier = Verifier {
            output_id: wasm_output_id.clone(),
            gas_limit: 99999999,
        };

        // Insert backend output.
        let unspend_core = OutputCore {
            data: OutputData::NativeToken(Amount(100)),
            locker: wasm_output_id.clone(),
            verifier: Some(verifier.clone()),
            owner: Vec::new(),
        };
        unlocker.cores.insert(unspend_output_id, unspend_core);

        let wasm_output = OutputCore {
            data: OutputData::Data(bin),
            locker: wasm_output_id.clone(),
            verifier: Some(verifier),
            owner: Vec::new(),
        };
        unlocker.cores.insert(wasm_output_id.clone(), wasm_output);

        unlocker
    }
}

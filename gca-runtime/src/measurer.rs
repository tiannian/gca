pub use pwasm_utils::rules::Rules;

use crate::Result;

pub fn inject_gas(code: &[u8], rules: impl Rules) -> Result<Vec<u8>> {
    let module = parity_wasm::deserialize_buffer(code)?;

    let module = pwasm_utils::inject_gas_counter(module, &rules, "_gca_gas")?;

    Ok(parity_wasm::serialize(module)?)
}

#[cfg(test)]
pub mod tests {
    use std::{env, fs, path::Path};

    use gca_core::OutputOperation;

    use crate::{host, Backend};

    pub fn test_gas<B: Backend>() {
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/log.wasm");
        let bin = fs::read(wasm_path).unwrap();
        let executor = crate::executor::tests::build_exeutor(bin);

        // instant host
        let log = host::Logger::<B::Memory>::new();
        let measurer = host::GcaMeasurer::<B::Memory>::new(10000);

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());
        unlock_backend.add_host("_gca_gas", measurer.clone());
        let code = executor.unlock_by_index(0, unlock_backend).unwrap();
        assert_eq!(code, 0);

        let mut operation_backend = B::new();
        operation_backend.add_host("_gca_log", log.clone());
        operation_backend.add_host("_gca_gas", measurer.clone());

        let operation = OutputOperation(0);
        let code = executor
            .verify_operation(operation, operation_backend)
            .unwrap();
        assert_eq!(code, 0);

        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        verifier_backend.add_host("_gca_gas", measurer.clone());

        let code = executor.verify_output(0, verifier_backend).unwrap();
        assert_eq!(code, 0);
        let mut verifier_backend = B::new();
        verifier_backend.add_host("_gca_log", log.clone());
        verifier_backend.add_host("_gca_gas", measurer.clone());

        let code = executor.verify_output(1, verifier_backend).unwrap();
        assert_eq!(code, 0);
    }
}

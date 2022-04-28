use pwasm_utils::rules::Rules;

use crate::Result;

pub fn inject_gas(code: &[u8], rules: impl Rules) -> Result<Vec<u8>> {
    let module = parity_wasm::deserialize_buffer(code)?;

    let module = pwasm_utils::inject_gas_counter(module, &rules, "_gca_gas")?;

    Ok(parity_wasm::serialize(module)?)
}

#[cfg(test)]
pub mod tests {
    use std::{collections::BTreeMap, env, fs, path::Path};

    use crate::{host, inject_gas, Backend, Instance};

    pub fn test_gas_unlocker<B: Backend>() {
        let env = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_path =
            Path::new(&env).join("../examples/target/wasm32-unknown-unknown/release/log.wasm");
        let bin = fs::read(wasm_path).unwrap();

        let set = pwasm_utils::rules::Set::new(1, BTreeMap::new());

        let gased_module = inject_gas(&bin, set).unwrap();

        let executor = crate::unlocker::tests::build_unlocker(gased_module);

        // instant host
        let log = host::Logger::<B::Instance>::default();
        let measurer = host::GcaMeasurer::<B::Instance>::new(10000);

        let mut unlock_backend = B::new();
        unlock_backend.add_host("_gca_log", log.clone());
        unlock_backend.add_host("_gca_gas", measurer.clone());

        let input = crate::unlocker::tests::build_input();

        let code = executor.unlock_input(&input, unlock_backend).unwrap();

        let any_host = code.1.get_host("_gca_gas").unwrap();

        println!("{:?}", any_host.type_id());

        let measurer = any_host
            .downcast_ref::<host::GcaMeasurer<B::Instance>>()
            .unwrap();

        println!("gas is: {}", measurer.gas());

        assert_eq!(code.0, 0);

   //      let mut operation_backend = B::new();
        // operation_backend.add_host("_gca_log", log.clone());
        // operation_backend.add_host("_gca_gas", measurer.clone());
//
//         let operation = OutputOperation(0);
        // let code = executor
        //     .verify_operation(operation, operation_backend)
        //     .unwrap();
        // assert_eq!(code.0, 0);
        //
        // // let gas operation_backend.
        //
        // let mut verifier_backend = B::new();
        // verifier_backend.add_host("_gca_log", log.clone());
        // verifier_backend.add_host("_gca_gas", measurer.clone());
        //
        // let code = executor
        //     .verify_output(0, verifier_backend)
        //     .unwrap()
        //     .unwrap();
        // assert_eq!(code.0, 0);
        // let mut verifier_backend = B::new();
        // verifier_backend.add_host("_gca_log", log.clone());
        // verifier_backend.add_host("_gca_gas", measurer.clone());
        //
        // let code = executor
        //     .verify_output(1, verifier_backend)
        //     .unwrap()
        //     .unwrap();
//         assert_eq!(code.0, 0);
    }
}

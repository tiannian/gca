use std::collections::BTreeMap;

use gca_core::{InputOperation, OutputCore, OutputData, OutputId, Transaction};

use crate::{Backend, Error, Instance, Module, ModuleInfo, Result, Val};

pub struct Verifier<'a> {
    pub cores: BTreeMap<OutputId, OutputCore>,
    pub reference: BTreeMap<u32, Vec<(String, OutputId)>>,
    pub tx: &'a Transaction,
}

impl<'a> Verifier<'a> {
    pub fn new(tx: &'a Transaction) -> Self {
        let mut reference: BTreeMap<u32, Vec<(String, OutputId)>> = BTreeMap::new();

        for input in &tx.inputs {
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
            cores: BTreeMap::new(),
            reference,
            tx,
        }
    }

    pub fn verify_output<B: Backend>(
        &self,
        index: usize,
        backend: B,
    ) -> Result<Option<(i32, B::Instance)>> {
        let output = self
            .tx
            .outputs
            .get(index)
            .ok_or(Error::ErrNoUnspentOutputPreLoad)?;

        if let Some(verifier) = &output.core.verifier {
            let i = self
                .cores
                .get(verifier)
                .ok_or(Error::ErrNoUnspentOutputPreLoad)?;
            if let OutputData::Data(code) = &i.data {
                self.verify(index as u32, code, backend).map(Some)
            } else {
                Err(Error::ErrOnlyDataCanLoad)
            }
        } else {
            Ok(None)
        }
    }

    pub fn verify<B: Backend>(
        &self,
        index: u32,
        code: &[u8],
        backend: B,
    ) -> Result<(i32, B::Instance)> {
        let mut deps = Vec::new();

        // Load dep module.
        if let Some(v) = self.reference.get(&index) {
            for (name, output_id) in v {
                let output = self
                    .cores
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

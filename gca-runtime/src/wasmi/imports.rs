use std::collections::BTreeMap;

use wasmi::{FuncRef, Signature};

use crate::{Host, Memory};

pub struct ModuleHostImport(pub(crate) BTreeMap<&'static str, usize>);

impl ModuleHostImport {
    pub fn new<M: Memory>(host: &impl Host<M>) -> Self {
        let mut inner = BTreeMap::new();

        let defines = host.resolve_functions();

        for i in 0..defines.len() {
            let define = &defines[i];
            inner.insert(define.name, i);
        }

        Self(inner)
    }
}

impl wasmi::ModuleImportResolver for ModuleHostImport {
    fn resolve_func(
        &self,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        let idx = self
            .0
            .get(field_name)
            .ok_or(wasmi::Error::Instantiation(format!(
                "Export {} not found",
                field_name
            )))?;

        // TODO: Check signature here.

        Ok(wasmi::FuncInstance::alloc_host(signature.clone(), *idx))
    }
}

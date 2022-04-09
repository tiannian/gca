use std::collections::BTreeMap;

use wasmi::{FuncRef, ModuleImportResolver, ModuleRef, Signature};

use crate::{Host, Instance};

pub enum ModuleHostImport {
    Host(BTreeMap<&'static str, usize>),
    ModuleRef(ModuleRef),
}

impl ModuleHostImport {
    pub fn new_host<M: Instance + 'static>(host: &dyn Host<M>, offset: usize) -> Self {
        let mut inner = BTreeMap::new();

        let defines = host.resolve_functions();

        for i in 0..defines.len() {
            let define = &defines[i];
            inner.insert(define.name, i + offset);
        }

        Self::Host(inner)
    }

    pub fn new_module(m: ModuleRef) -> Self {
        Self::ModuleRef(m)
    }

    pub fn get_host_idxs(&self) -> Option<&BTreeMap<&'static str, usize>> {
        match self {
            ModuleHostImport::Host(idx) => Some(idx),
            ModuleHostImport::ModuleRef(_) => None,
        }
    }
}

impl wasmi::ModuleImportResolver for ModuleHostImport {
    fn resolve_func(
        &self,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        match self {
            ModuleHostImport::Host(h) => {
                let idx = h
                    .get(field_name)
                    .ok_or(wasmi::Error::Instantiation(format!(
                        "Export {} not found",
                        field_name
                    )))?;

                Ok(wasmi::FuncInstance::alloc_host(signature.clone(), *idx))
            }
            ModuleHostImport::ModuleRef(m) => m.resolve_func(field_name, signature),
        }
    }

    fn resolve_table(
        &self,
        field_name: &str,
        table_type: &wasmi::TableDescriptor,
    ) -> Result<wasmi::TableRef, wasmi::Error> {
        match self {
            ModuleHostImport::Host(_) => Err(wasmi::Error::Instantiation(format!(
                "no field: {}",
                field_name
            ))),
            ModuleHostImport::ModuleRef(m) => m.resolve_table(field_name, table_type),
        }
    }

    fn resolve_global(
        &self,
        field_name: &str,
        global_type: &wasmi::GlobalDescriptor,
    ) -> Result<wasmi::GlobalRef, wasmi::Error> {
        match self {
            ModuleHostImport::Host(_) => Err(wasmi::Error::Instantiation(format!(
                "no field: {}",
                field_name
            ))),
            ModuleHostImport::ModuleRef(m) => m.resolve_global(field_name, global_type),
        }
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        memory_type: &wasmi::MemoryDescriptor,
    ) -> Result<wasmi::MemoryRef, wasmi::Error> {
        match self {
            ModuleHostImport::Host(_) => Err(wasmi::Error::Instantiation(format!(
                "no field: {}",
                field_name
            ))),
            ModuleHostImport::ModuleRef(m) => m.resolve_memory(field_name, memory_type),
        }
    }
}

pub struct HostImports(pub(crate) BTreeMap<String, ModuleHostImport>);

impl HostImports {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add_module(&mut self, name: &str, module: ModuleHostImport) {
        self.0.insert(String::from(name), module);
    }
}

impl wasmi::ImportResolver for HostImports {
    fn resolve_func(
        &self,
        _module_name: &str,
        field_name: &str,
        signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        let module = self
            .0
            .get(_module_name)
            .ok_or(wasmi::Error::Instantiation(format!(
                "Export module {} not found",
                _module_name
            )))?;

        module.resolve_func(field_name, signature)
    }

    fn resolve_table(
        &self,
        module_name: &str,
        field_name: &str,
        descriptor: &wasmi::TableDescriptor,
    ) -> Result<wasmi::TableRef, wasmi::Error> {
        let module = self
            .0
            .get(module_name)
            .ok_or(wasmi::Error::Instantiation(format!(
                "Export module {} not found",
                module_name
            )))?;

        module.resolve_table(field_name, descriptor)
    }

    fn resolve_global(
        &self,
        module_name: &str,
        field_name: &str,
        descriptor: &wasmi::GlobalDescriptor,
    ) -> Result<wasmi::GlobalRef, wasmi::Error> {
        let module = self
            .0
            .get(module_name)
            .ok_or(wasmi::Error::Instantiation(format!(
                "Export module {} not found",
                module_name
            )))?;

        module.resolve_global(field_name, descriptor)
    }

    fn resolve_memory(
        &self,
        module_name: &str,
        field_name: &str,
        descriptor: &wasmi::MemoryDescriptor,
    ) -> Result<wasmi::MemoryRef, wasmi::Error> {
        let module = self
            .0
            .get(module_name)
            .ok_or(wasmi::Error::Instantiation(format!(
                "Export module {} not found",
                module_name
            )))?;

        module.resolve_memory(field_name, descriptor)
    }
}

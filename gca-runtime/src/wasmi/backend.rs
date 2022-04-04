use std::collections::BTreeMap;

use wasmi::ModuleInstance;

use crate::{Backend, Host, ModuleInfo, Result};

use super::{
    HostImports, ModuleHostImport, WasmiExternal, WasmiInstance, WasmiMemory, WasmiModule,
};

pub struct WasmiBackend {
    pub(crate) host_idxs: BTreeMap<usize, (usize, &'static str)>,
    pub(crate) hosts: Vec<(String, Box<dyn Host<WasmiMemory>>)>,
}

impl Backend for WasmiBackend {
    type Memory = WasmiMemory;

    type Module = WasmiModule;

    type Instance = WasmiInstance;

    fn new() -> Self {
        Self {
            host_idxs: BTreeMap::new(),
            hosts: Vec::new(),
        }
    }

    fn add_host(&mut self, name: &str, host: impl Host<Self::Memory>) {
        self.hosts.push((String::from(name), Box::new(host)));
    }

    fn instance(
        &mut self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance> {
        let external = WasmiExternal {};

        let mut imports = HostImports::new();

        for (name, host) in &self.hosts {
            let import = ModuleHostImport::new_host(host.as_ref());

            imports.add_module(&name, import);
        }

        for mi in deps {
            let instance = ModuleInstance::new(&mi.module.m, &imports)?.assert_no_start();

            imports.add_module(mi.name, ModuleHostImport::new_module(instance));
        }

        let instance = ModuleInstance::new(&module.m, &imports)?.assert_no_start();

        Ok(WasmiInstance { external, instance })
    }
}

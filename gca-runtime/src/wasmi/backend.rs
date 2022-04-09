use std::collections::BTreeMap;

use wasmi::ModuleInstance;

use crate::{Backend, Host, ModuleInfo, Result};

use super::{
    HostImports, ModuleHostImport, WasmiExternal, WasmiInstance, WasmiMemory, WasmiModule,
};

pub struct WasmiBackend {
    pub(crate) host_idxs: BTreeMap<usize, (usize, &'static str)>,
    pub(crate) hosts: Vec<(String, Box<dyn Host<WasmiInstance>>)>,
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

    fn add_host(&mut self, name: &str, host: impl Host<Self::Instance>) {
        self.hosts.push((String::from(name), Box::new(host)));
    }

    fn instance(
        self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance> {
        let mut this = self;

        let mut imports = HostImports::new();

        for i in 0..this.hosts.len() {
            let (name, host) = &this.hosts[i];

            let import = ModuleHostImport::new_host(host.as_ref(), this.host_idxs.len());

            if let Some(idxs) = import.get_host_idxs() {
                for (name, idx) in idxs {
                    this.host_idxs.insert(*idx, (i, name));
                }
            }

            imports.add_module(&name, import);
        }

        log::info!("Host index map for wasmi:");
        log::info!("{:#?}", this.host_idxs);

        for mi in deps {
            let instance = ModuleInstance::new(&mi.module.m, &imports)?.assert_no_start();

            imports.add_module(mi.name, ModuleHostImport::new_module(instance));
        }

        let instance = ModuleInstance::new(&module.m, &imports)?.assert_no_start();

        for host in &mut this.hosts {
            host.1.set_instance(WasmiInstance {
                instance: instance.clone(),
                external: None,
            });
        }

        let external = WasmiExternal {
            host_idxs: this.host_idxs,
            hosts: this.hosts,
        };

        Ok(WasmiInstance {
            external: Some(external),
            instance,
        })
    }
}

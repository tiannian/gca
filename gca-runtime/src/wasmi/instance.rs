use crate::{Host, Instance, Result, Val};

use super::{WasmiMemory, WasmiModule};

mod wrapper {
    use std::collections::BTreeMap;

    use crate::Host;

    pub struct HostWrapper {
        pub(crate) defines_idx: BTreeMap<&'static str, usize>,
    }

    impl HostWrapper {
        pub fn new<H: Host>(h: &H) -> Self {
            let defines = h.resolve_functions();

            let mut defines_idx = BTreeMap::new();

            for i in 0..defines.len() {
                let name = defines[i].name;
                defines_idx.insert(name, i);
            }

            Self { defines_idx }
        }
    }

    impl wasmi::ModuleImportResolver for HostWrapper {
        fn resolve_func(
            &self,
            field_name: &str,
            signature: &wasmi::Signature,
        ) -> Result<wasmi::FuncRef, wasmi::Error> {
            if let Some(idx) = self.defines_idx.get(field_name) {
                // TODO: Check signature here.
                let func = wasmi::FuncInstance::alloc_host(signature.clone(), *idx);

                Ok(func)
            } else {
                Err(wasmi::Error::Instantiation(format!(
                    "No host function {}",
                    field_name
                )))
            }
        }
    }
}

pub struct WasmiInstance<'a, H> {
    pub(crate) instance: wasmi::ModuleRef,
    pub(crate) host: &'a mut H,
}

impl<'a, H> Instance<H> for WasmiInstance<'a, H>
where
    H: Host,
{
    type Memory = WasmiMemory;

    type Module = WasmiModule;

    //     fn new(module: &Self::Module, deps: &[(&str, Self)], host_name: &str, host: H) -> Result<Self> {
    // let mut imports = wasmi::ImportsBuilder::new();
    //
    // for (s, i) in deps {
    //     imports.push_resolver(*s, &i.instance);
    // }
    //
    // let h = wrapper::HostWrapper::new(&host);
    //
    // imports.push_resolver(host_name, &h);
    //
    // let instance = wasmi::ModuleInstance::new(&module.m, &imports)?.assert_no_start();
    //
    // Ok(WasmiInstance { instance })
    //     }

    fn call_func(&self, name: &str, parmas: &[Val]) -> Result<Option<Val>> {
        let args: Vec<wasmi::RuntimeValue> = parmas
            .iter()
            .map(|e| wasmi::RuntimeValue::from(e.clone()))
            .collect();

        // self.instance.invoke_export(name, &args, &mut h)?;

        Ok(None)
    }

    fn get_memory(&self, name: &str) -> Option<Self::Memory> {
        if let Some(wasmi::ExternVal::Memory(m)) = self.instance.export_by_name(name) {
            Some(WasmiMemory { m })
        } else {
            None
        }
    }
}

impl From<Val> for wasmi::RuntimeValue {
    fn from(e: Val) -> Self {
        match e {
            Val::I32(i) => Self::I32(i),
            Val::I64(i) => Self::I64(i),
            Val::F32(i) => Self::F32(wasmi::nan_preserving_float::F32::from_bits(i)),
            Val::F64(i) => Self::F64(wasmi::nan_preserving_float::F64::from_bits(i)),
        }
    }
}

impl From<wasmi::RuntimeValue> for Val {
    fn from(e: wasmi::RuntimeValue) -> Self {
        match e {
            wasmi::RuntimeValue::I32(i) => Val::I32(i),
            wasmi::RuntimeValue::I64(i) => Val::I64(i),
            wasmi::RuntimeValue::F32(i) => Val::F32(i.to_bits()),
            wasmi::RuntimeValue::F64(i) => Val::F64(i.to_bits()),
        }
    }
}

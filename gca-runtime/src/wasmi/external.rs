use std::{collections::BTreeMap, fmt::Display};

use wasmi::{Error, HostError, RuntimeArgs, RuntimeValue, Trap, TrapKind};

use crate::{Host, Val};

use super::WasmiMemory;

#[derive(Debug)]
pub enum ExternalError {
    NoTargetIndex,
}

impl Display for ExternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HostError for ExternalError {}

pub struct WasmiExternal {
    pub(crate) host_idxs: BTreeMap<usize, (usize, &'static str)>,
    pub(crate) hosts: Vec<(String, Box<dyn Host<WasmiMemory>>)>,
}

impl wasmi::Externals for WasmiExternal {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        let (module_idx, name) =
            self.host_idxs
                .get(&index)
                .ok_or(Trap::new(TrapKind::Host(Box::new(
                    ExternalError::NoTargetIndex,
                ))))?;

        let host = self
            .hosts
            .get_mut(*module_idx)
            .ok_or(Trap::new(TrapKind::Host(Box::new(ExternalError::NoTargetIndex))))?;

        let mut values = Vec::new();

        for arg in args.as_ref() {
            let val: Val = arg.clone().into();

            values.push(val);
        }

        // host.1.call_func(name, &values)?;

        Ok(None)
    }
}

use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
};

use wasmi::{HostError, RuntimeArgs, RuntimeValue, Trap, TrapKind};

use crate::{Host, Val};

use super::WasmiInstance;

#[derive(Debug)]
pub enum ExternalError {
    NoTargetIndex,
    FunctionCallErr(Box<dyn Debug + Sync + Send>),
}

impl Display for ExternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HostError for ExternalError {}

pub struct WasmiExternal {
    pub(crate) host_idxs: BTreeMap<usize, (usize, &'static str)>,
    pub(crate) hosts: Vec<(String, Box<dyn Host<WasmiInstance>>)>,
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
                .ok_or_else(|| Trap::new(TrapKind::Host(Box::new(
                    ExternalError::NoTargetIndex,
                ))))?;

        let host = self
            .hosts
            .get_mut(*module_idx)
            .ok_or_else(|| Trap::new(TrapKind::Host(Box::new(
                ExternalError::NoTargetIndex,
            ))))?;

        let mut values = Vec::new();

        for arg in args.as_ref() {
            let val: Val = (*arg).into();

            values.push(val);
        }

        let res = host
            .1
            .call_func(name, &values)
            .map_err(|e| ExternalError::FunctionCallErr(e))?
            .map(|v| v.into());

        Ok(res)
    }
}

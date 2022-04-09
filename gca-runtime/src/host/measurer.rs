use std::{fmt::Debug, marker::PhantomData};

use crate::{FuncDefine, Host, Instance, Val, ValTy};

pub struct GcaMeasurer<M> {
    gas: u64,
    gas_limit: u64,
    func_def: Vec<FuncDefine>,
    marker_b: PhantomData<M>,
}

impl<M> Clone for GcaMeasurer<M> {
    fn clone(&self) -> Self {
        Self {
            gas: self.gas,
            gas_limit: self.gas_limit,
            func_def: self.func_def.clone(),
            marker_b: PhantomData,
        }
    }
}

impl<M> GcaMeasurer<M> {
    pub fn new(gas_limit: u64) -> Self {
        let f = FuncDefine {
            name: "_gca_gas",
            parmas: vec![ValTy::I32],
            ret: None,
        };

        let func_def = vec![f];

        Self {
            gas: 0,
            gas_limit,
            func_def,
            marker_b: PhantomData,
        }
    }

    pub fn gas(&self) -> u64 {
        self.gas
    }
}

#[derive(Debug)]
enum GcaMeasurerHostError {
    ErrArgumentsFormat,
    ErrCalledName,
    ExccedGasLimit,
}

impl From<GcaMeasurerHostError> for Box<dyn Debug + Send + Sync> {
    fn from(e: GcaMeasurerHostError) -> Self {
        Box::new(e)
    }
}

impl<M: Instance + 'static> Host<M> for GcaMeasurer<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, _instance: M) {}

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn Debug + Send + Sync>> {
        if name != "gas" {
            return Err(GcaMeasurerHostError::ErrCalledName.into());
        }

        if args.len() != 1 {
            return Err(GcaMeasurerHostError::ErrArgumentsFormat.into());
        }

        if let Some(Val::I32(i)) = args.get(0) {
            // TODO: Add exccess to exit Execute.
            let step_gas = *i as u64;

            self.gas += step_gas;

            if self.gas > self.gas_limit {
                return Err(GcaMeasurerHostError::ExccedGasLimit.into());
            }
        } else {
            return Err(GcaMeasurerHostError::ErrArgumentsFormat.into());
        }

        Ok(None)
    }
}

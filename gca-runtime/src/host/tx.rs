use std::{fmt::Debug, sync::Arc};

use gca_core::Transaction;

use crate::{FuncDefine, Host, Instance, Val, ValTy};

pub struct TxFetcher<M> {
    func_def: Arc<Vec<FuncDefine>>,
    instance: Option<M>,
    tx: Transaction,
}

impl<M> Clone for TxFetcher<M> {
    fn clone(&self) -> Self {
        Self {
            func_def: self.func_def.clone(),
            instance: None,
            tx: self.tx.clone(),
        }
    }
}

impl<M> TxFetcher<M> {
    pub fn new(tx: Transaction) -> Self {
        let _get_txhash = FuncDefine {
            name: "_get_txhash",
            parmas: vec![ValTy::I32],
            ret: None,
        };

        let _input_get_count = FuncDefine {
            name: "_input_get_count",
            parmas: vec![],
            ret: Some(ValTy::I32),
        };

        let _input_get_operation_by_index = FuncDefine {
            name: "_input_get_operation_by_index",
            parmas: vec![ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let _input_is_reference_input = FuncDefine {
            name: "_input_is_reference_input",
            parmas: vec![ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let _input_get_reference_by_index = FuncDefine {
            name: "_input_get_reference_by_index",
            parmas: vec![ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let _input_get_reference_name_by_index = FuncDefine {
            name: "_input_get_reference_name_by_index",
            parmas: vec![ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let _input_get_output_id_by_index = FuncDefine {
            name: "_input_get_output_id_by_index",
            parmas: vec![ValTy::I32, ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let _input_get_unlock_data_by_index = FuncDefine {
            name: "_input_get_unlock_data_by_index",
            parmas: vec![ValTy::I32, ValTy::I32],
            ret: Some(ValTy::I32),
        };

        let func_def = Arc::new(vec![
            _get_txhash,
            _input_get_count,
            _input_get_operation_by_index,
            _input_is_reference_input,
            _input_get_reference_by_index,
            _input_get_reference_name_by_index,
            _input_get_output_id_by_index,
            _input_get_unlock_data_by_index,
        ]);

        Self {
            func_def,
            instance: None,
            tx,
        }
    }
}

#[derive(Debug)]
enum TxFetcherHostError {}

impl From<TxFetcherHostError> for Box<dyn Debug + Send + Sync> {
    fn from(e: TxFetcherHostError) -> Self {
        Box::new(e)
    }
}

impl<M: Instance + 'static> Host<M> for TxFetcher<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, _instance: M) {}

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn Debug + Send + Sync>> {
        Ok(None)
    }
}

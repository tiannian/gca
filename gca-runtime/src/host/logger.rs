use std::{any::Any, fmt::Debug, string::FromUtf8Error, sync::Arc};

use crate::{FuncDefine, Host, Instance, Memory, Val, ValTy};

#[derive(Debug)]
pub enum LoggerError {
    NoMethod,
    NoArgument,
    NoMemory,
    ErrWasmType,
    UnknownLevelIndex,
    FromUtf8Error,
    MemoryReadError,
}

impl From<FromUtf8Error> for LoggerError {
    fn from(_: FromUtf8Error) -> Self {
        LoggerError::FromUtf8Error
    }
}

impl From<LoggerError> for Box<dyn Debug + Send + Sync> {
    fn from(e: LoggerError) -> Self {
        Box::new(e)
    }
}

pub struct Logger<M> {
    func_def: Arc<Vec<FuncDefine>>,
    instance: Option<M>,
}

impl<M> Clone for Logger<M> {
    fn clone(&self) -> Self {
        Self {
            func_def: self.func_def.clone(),
            instance: None,
        }
    }
}

impl<M> Logger<M> {
    pub fn new() -> Self {
        let f = FuncDefine {
            name: "_gca_log",
            parmas: vec![
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
                ValTy::I32,
            ],
            ret: Some(ValTy::I32),
        };

        let func_def = Arc::new(vec![f]);

        Self {
            func_def,
            instance: None,
        }
    }
}

impl<M: Instance + 'static> Host<M> for Logger<M> {
    fn resolve_functions(&self) -> &[FuncDefine] {
        &self.func_def
    }

    fn set_instance(&mut self, instance: M) {
        self.instance = Some(instance);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn call_func(
        &mut self,
        name: &str,
        args: &[Val],
    ) -> std::result::Result<Option<Val>, Box<dyn std::fmt::Debug + Sync + Send>> {
        if name == "_gca_log" {
            let level = args.get(0).ok_or(LoggerError::NoArgument)?;
            let target_ptr = args.get(1).ok_or(LoggerError::NoArgument)?;
            let target_len = args.get(2).ok_or(LoggerError::NoArgument)?;
            let file_ptr = args.get(3).ok_or(LoggerError::NoArgument)?;
            let file_len = args.get(4).ok_or(LoggerError::NoArgument)?;
            let line = args.get(5).ok_or(LoggerError::NoArgument)?;
            let message_ptr = args.get(6).ok_or(LoggerError::NoArgument)?;
            let message_len = args.get(7).ok_or(LoggerError::NoArgument)?;

            let memory = self
                .instance
                .as_ref()
                .ok_or(LoggerError::NoMemory)?
                .get_memory("memory")
                .ok_or(LoggerError::NoMemory)?;

            let level = val_to_level(level)?;
            let target = get_string(target_ptr, target_len, &memory)?;
            let file = get_string(file_ptr, file_len, &memory)?;
            let message = get_string(message_ptr, message_len, &memory)?;
            let line = val_to_u32(line)?;

            log::log!(target: &target, level, "{}:{} {}", file, line, message);

            Ok(None)
        } else {
            Err(Box::new(LoggerError::NoMethod))
        }
    }
}

fn val_to_u32(v: &Val) -> Result<u32, LoggerError> {
    match v {
        Val::I32(i) => Ok(*i as u32),
        _ => Err(LoggerError::ErrWasmType),
    }
}

fn val_to_i32(v: &Val) -> Result<i32, LoggerError> {
    match v {
        Val::I32(i) => Ok(*i),
        _ => Err(LoggerError::ErrWasmType),
    }
}

fn get_string(ptr: &Val, len: &Val, memory: &impl Memory) -> Result<String, LoggerError> {
    let ptr = val_to_i32(ptr)?;
    let len = val_to_i32(len)?;

    let mut buf = Vec::<u8>::with_capacity(len as usize);

    buf.resize(len as usize, 0);

    memory
        .read(ptr as usize, &mut buf)
        .map_err(|_| LoggerError::MemoryReadError)?;

    Ok(String::from_utf8(buf)?)
}

fn val_to_level(level: &Val) -> Result<log::Level, LoggerError> {
    let level = if let Val::I32(level) = level {
        match level {
            1 => log::Level::Error,
            2 => log::Level::Warn,
            3 => log::Level::Info,
            4 => log::Level::Debug,
            5 => log::Level::Trace,
            _ => return Err(LoggerError::UnknownLevelIndex),
        }
    } else {
        return Err(LoggerError::ErrWasmType);
    };

    Ok(level)
}

mod module;
pub use module::*;

mod instance;
pub use instance::*;

mod memory;
pub use memory::*;

mod backend;
pub use backend::*;

mod external;
pub use external::*;

mod imports;
pub use imports::*;

#[cfg(test)]
mod test {
    use super::WasmiBackend;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_empty() {
        init();

        crate::unlocker::tests::test_empty_unlocker::<WasmiBackend>();
    }

    #[test]
    fn test_log() {
        init();

        crate::unlocker::tests::test_log_unlocker::<WasmiBackend>();
    }

    #[test]
    fn test_gas() {
        init();

        crate::measurer::tests::test_gas_unlocker::<WasmiBackend>();
    }

    #[test]
    fn test_chain_id() {
        init();

        crate::unlocker::tests::test_chain_id_unlocker::<WasmiBackend>();
    }
}

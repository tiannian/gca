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

        crate::executor::tests::test_empty::<WasmiBackend>();
    }

    #[test]
    fn test_log() {
        init();

        crate::executor::tests::test_log::<WasmiBackend>();
    }

    #[test]
    fn test_gas() {
        init();

        crate::measurer::tests::test_gas::<WasmiBackend>();
    }

    #[test]
    fn test_chain_id() {
        init();

        crate::executor::tests::test_chain_id::<WasmiBackend>();
    }
}

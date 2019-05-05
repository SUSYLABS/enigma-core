#![feature(tool_lints)]
#![warn(clippy::all)]
#![feature(try_from)]
#![feature(int_to_from_bytes)]
#![warn(unused_extern_crates)]

extern crate dirs;
pub extern crate rocksdb;
pub extern crate sgx_types;
extern crate sgx_urts;
#[macro_use]
extern crate lazy_static;
pub extern crate futures;
extern crate rmp_serde;
pub extern crate serde_json;
extern crate tokio_zmq;
extern crate zmq;
#[macro_use]
extern crate failure;
pub extern crate enigma_tools_u;
extern crate enigma_crypto;
extern crate enigma_types;
extern crate rustc_hex as hex;
extern crate lru_cache;
#[macro_use]
extern crate serde;
#[macro_use]
pub extern crate log;
#[macro_use]
pub extern crate log_derive;
pub extern crate structopt;
pub extern crate simplelog;

pub mod common_u;
pub mod db;
pub mod esgx;
pub mod evm_u;
pub mod km_u;
pub mod networking;
pub mod wasm_u;
pub mod cli;

#[cfg(feature = "cross-test-utils")]
pub mod cross_test_utils {
    use super::*;

}

#[cfg(test)]
mod tests {
    extern crate tempfile;
    use crate::esgx::general::init_enclave_wrapper;
    use sgx_types::*;
    use crate::db::DB;
    use enigma_types::{RawPointer, ResultStatus};
    use simplelog::TermLogger;
    use log::LevelFilter;
    use self::tempfile::TempDir;

    extern "C" {
        fn ecall_run_tests(eid: sgx_enclave_id_t, db_ptr: *const RawPointer, result: *mut ResultStatus) -> sgx_status_t;
    }

    /// It's important to save TempDir too, because when it gets dropped the directory will be removed.
    fn create_test_db() -> (DB, TempDir) {
        let tempdir = tempfile::tempdir().unwrap();
        let db = DB::new(tempdir.path(), true).unwrap();
        (db, tempdir)
    }

    #[allow(dead_code)]
    pub fn log_to_stdout(level: Option<LevelFilter>) {
        let level = level.unwrap_or_else(|| LevelFilter::max());
        TermLogger::init(level, Default::default()).unwrap();
    }

    #[test]
    pub fn test_enclave_internal() {
        let (mut db, _dir) = create_test_db();
        let enclave = init_enclave_wrapper().unwrap();
        let db_ptr = unsafe { RawPointer::new_mut(&mut db) };
        let mut result: ResultStatus = ResultStatus::Ok;
        let ret = unsafe { ecall_run_tests(enclave.geteid(), &db_ptr as *const RawPointer, &mut result) };

        assert_eq!(ret, sgx_status_t::SGX_SUCCESS);
        assert_eq!(result,ResultStatus::Ok);
    }
}

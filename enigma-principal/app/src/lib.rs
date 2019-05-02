#![feature(integer_atomics)]
#![feature(try_from)]
#![feature(arbitrary_self_types)]

#[macro_use]
extern crate colour;
extern crate dirs;
extern crate enigma_crypto;
extern crate enigma_tools_m;
extern crate enigma_tools_u;
extern crate enigma_types;
extern crate ethabi;
#[macro_use]
extern crate failure;
extern crate jsonrpc_http_server;
extern crate log;
#[macro_use]
extern crate log_derive;
extern crate rlp;
extern crate rustc_hex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sgx_types;
extern crate sgx_urts;
extern crate structopt;
extern crate url;
extern crate web3;
extern crate rmp_serde;
// enigma modules
pub mod boot_network;
pub mod cli;
pub mod common_u;
pub mod epoch_u;
pub mod esgx;

#[cfg(test)]
mod tests {
    extern crate log;
    extern crate sgx_types;
    use enigma_tools_u::common_u::logging::TermLogger;
    use esgx::general::init_enclave_wrapper;
    use log::LevelFilter;
    use sgx_types::{sgx_enclave_id_t, sgx_status_t};

    extern "C" {
        fn ecall_run_tests(eid: sgx_enclave_id_t) -> sgx_status_t;
    }

    #[allow(dead_code)]
    pub fn log_to_stdout(level: Option<LevelFilter>) {
        let level = level.unwrap_or_else(|| LevelFilter::max());
        TermLogger::init(level, Default::default()).unwrap();
    }

    #[test]
    pub fn test_enclave_internal() {
        // initiate the enclave
        let enclave = match init_enclave_wrapper() {
            Ok(r) => {
                println!("[+] Init Enclave Successful {}!", r.geteid());
                r
            }
            Err(x) => {
                println!("[-] Init Enclave Failed {}!", x.as_str());
                assert_eq!(0, 1);
                return;
            }
        };

        println!("ecall_run_tests, thread: {:?}", std::thread::current().id());
        println!("ecall_run_tests, thread: {:?}", std::thread::current().name());

        let ret = unsafe { ecall_run_tests(enclave.geteid()) };
        assert_eq!(ret, sgx_status_t::SGX_SUCCESS);
        enclave.destroy();
    }
}

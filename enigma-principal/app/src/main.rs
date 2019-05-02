extern crate enigma_principal_app;
extern crate enigma_tools_u;
extern crate dirs;
extern crate structopt;

use enigma_principal_app::{esgx, cli::{self, options::Opt}};
use enigma_tools_u::common_u::logging::{self, CombinedLogger};
use structopt::StructOpt;


fn main() {
    let opt: Opt = Opt::from_args();
    println!("CLI params: {:?}", opt);

    let datadir = dirs::home_dir().unwrap().join(".enigma");
    let loggers = logging::get_logger(opt.debug_stdout, datadir.clone(), opt.verbose).expect("Failed Creating the loggers");
    CombinedLogger::init(loggers).expect("Failed initializing the logger");

    // init enclave
    let enclave = match esgx::general::init_enclave_wrapper() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        }
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        }
    };

    // run THE app
    let eid = enclave.geteid();
    cli::app::start(eid, opt).unwrap();

    // drop enclave when done
    enclave.destroy();
}
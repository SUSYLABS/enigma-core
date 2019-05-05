use boot_network::{
    keys_provider_http::{PrincipalHttpServer, StateKeyRequest},
    principal_manager::{self, PrincipalManager, ReportManager, Sampler},
};
use cli;
use enigma_tools_u::{esgx::general::storage_dir, web3_utils::enigma_contract::EnigmaContract};
use epoch_u::epoch_provider::EpochProvider;
use esgx::general::ENCLAVE_DIR;
use failure::Error;
use sgx_types::sgx_enclave_id_t;
use std::{fs::File, io::prelude::*, path::Path, sync::Arc};
use cli::options::Opt;


const GAS_LIMIT: u64 = 5_999_999;


#[logfn(INFO)]
pub fn start(eid: sgx_enclave_id_t, opt: Opt) -> Result<(), Error> {
    // Get CLI Options.
    // Load the config from Options.
    let mut principal_config = PrincipalManager::load_config(&opt.principal_config)?;

    let report_manager = ReportManager::new(principal_config.clone(), eid)?;
    let signing_address = "";

    if opt.info { // Print information.
        cli::options::print_info(&signing_address);

    } else if opt.sign_address { // Print the signing address.
        let mut path = storage_dir(ENCLAVE_DIR)?;
        path.push("principal-sign-addr.txt");
        let mut file = File::create(path.clone())?;
        let prefixed_signing_address = format!("0x{}", signing_address);
        file.write_all(prefixed_signing_address.as_bytes())?;
        println!("Wrote signing address: {:?} in file: {:?}", prefixed_signing_address, path);

    } else if opt.deploy { // Deploy the contract.
        unimplemented!("Self-deploy mode not yet implemented. Fix issues with linked libraries in the Enigma contract.");

    } else {
        println!("[Mode:] run node NO DEPLOY.");

        // step1 : build the config of the principal node
        // optional : set time limit for the principal node
        let contract_address = opt.contract_address.unwrap_or_else(|| principal_config.enigma_contract_address.clone());
        let enigma_contract = Arc::new(EnigmaContract::from_deployed(
            &contract_address,
            Path::new(&principal_config.enigma_contract_path),
            Some(&principal_config.account_address),
            &principal_config.url,
        )?);

        principal_config.max_epochs = if opt.time_to_live > 0 { Some(opt.time_to_live) } else { None };

        let principal: PrincipalManager = PrincipalManager::new(principal_config.clone(), enigma_contract, report_manager)?;
        println!("Connected to the Enigma contract: {:?} with account: {:?}", &contract_address, principal.get_account_address());

        // step2 optional - run miner to simulate blocks
        let join_handle = if opt.mine > 0 {
            Some(principal_manager::run_miner(principal.get_account_address(), principal.get_web3(), opt.mine))
        } else {
            None
        };

        let eid_safe = Arc::new(0);
        let epoch_provider = EpochProvider::new(eid_safe, principal.contract.clone())?;

        if opt.reset_epoch_state {
            epoch_provider.reset_epoch_state()?;
        }

        // step3 : run the principal manager
        if opt.register { // Just register.
            match principal.verify_identity_or_register(GAS_LIMIT)? {
                Some(tx) => println!("Registered Principal with tx: {:?}", tx),
                None => println!("Principal already registered"),
            };

        } else if opt.set_worker_params { // Emit random
            let block_number = principal.get_block_number()?;
            let tx = epoch_provider.set_worker_params(block_number, GAS_LIMIT, principal_config.confirmations as usize)?;
            println!("The setWorkersParams tx: {:?}", tx);

        } else if opt.get_state_keys.is_some() { // Get specific state keys.
            let request: StateKeyRequest = serde_json::from_str(&opt.get_state_keys.unwrap())?;
            let response = PrincipalHttpServer::get_state_keys(Arc::new(epoch_provider), request)?;
            println!("The getStateKeys response: {}", serde_json::to_string(&response)?);

        } else { // Just run the principal node and start listening for requests.
            principal.run(false, GAS_LIMIT).unwrap();
        }
        if let Some(t) = join_handle {
            t.join().unwrap();
        }
    }

    Ok(())
}

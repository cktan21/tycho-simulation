extern crate tycho_simulation;
pub mod utils;

use std::{
    fs::File,
    io::{BufWriter, Write},
    env,
    str::FromStr,
};
use alloy::{
    providers::{ProviderBuilder, RootProvider},
    transports::BoxTransport,
};
use dotenv::dotenv;
use tokio::runtime::Runtime;
// use super::{
//     super::account_storage::{AccountStorage, StateUpdate},
//     engine_db_interface::EngineDatabaseInterface,
// };
// use crate::{OverriddenSimulationDB, SimulationDB};

// Mocked account address for testing
const ACCOUNT_ADDRESS: &str = "0x70a3b8f156c42d2295270f124353668a41158b22";

fn main() {
    // Load environment variables
    dotenv().ok();

    // Get the RPC URL from .env
    let rpc_url = env::var("RPC_URL").expect("Missing RPC_URL in .env");

    // Initialize Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let client = rt.block_on(async {
        ProviderBuilder::new()
            .on_builtin(&rpc_url)
            .await
            .unwrap()
    });

    // Create a SimulationDB instance
    let db = SimulationDB::new(
        Arc::new(client),
        Some(Arc::new(rt)),
        None,
    );

    // Define the address to query
    let address = Address::from_str(ACCOUNT_ADDRESS).expect("Invalid address");

    // Query and store data in temp storage
    let _ = db.basic_ref(address).unwrap();
    let _ = db.storage_ref(address, U256::zero()).unwrap();

    // Now, extract and print all stored values to a file
    print_to_file(&db, address);
}

fn print_to_file(db: &SimulationDB<RootProvider<BoxTransport>>, address: Address) {
    // Open a file to write the output
    let file = File::create("output.txt").expect("Failed to create file");
    let mut writer = BufWriter::new(file);

    // Write basic info
    if let Some(account_info) = db.account_storage.read().unwrap().get_account_info(&address) {
        writeln!(writer, "Account Info for {}: {:?}", address, account_info).unwrap();
    }

    // Write storage values
    writeln!(writer, "\nStorage Values for {}:", address).unwrap();
    for index in 0..10 {
        if let Ok(value) = db.storage_ref(address, U256::from(index)) {
            writeln!(writer, "Slot {}: {}", index, value).unwrap();
        }
    }

    println!("Data written to 'output.txt'");
}
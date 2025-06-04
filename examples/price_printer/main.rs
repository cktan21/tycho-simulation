use std::{env, str::FromStr};

use clap::Parser;
use futures::{future::select_all, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tycho_client::feed::component_tracker::ComponentFilter;
use tycho_common::models::Chain;
use tycho_simulation::{
    evm::{
        engine_db::tycho_db::PreCachedDB,
        protocol::{
            ekubo::state::EkuboState,
            filters::{balancer_pool_filter, curve_pool_filter, uniswap_v4_pool_with_hook_filter},
            uniswap_v2::state::UniswapV2State,
            uniswap_v3::state::UniswapV3State,
            uniswap_v4::state::UniswapV4State,
            vm::state::EVMPoolState,
        },
        stream::ProtocolStreamBuilder,
    },
    protocol::models::BlockUpdate,
    utils::load_all_tokens,
};

use std::fs;
use std::path::Path;
use alloy_primitives::U256;
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct RawUniswapV2Data {
    uniswap_v2: Vec<RawPool>,
}

#[derive(Debug, Deserialize)]
struct RawPool {
    component_id: String,
    attributes: HashMap<String, String>,
    balances: HashMap<String, String>,
}

fn parse_uniswap_v2_data<P: AsRef<Path>>(path: P) -> Result<Vec<UniswapV2State>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    let raw_data: RawUniswapV2Data = serde_json::from_str(&data)?;

    let mut states = Vec::new();

    for pool in raw_data.uniswap_v2 {
        let reserve0_hex = pool.attributes.get("reserve0").ok_or("Missing reserve0")?;
        let reserve1_hex = pool.attributes.get("reserve1").ok_or("Missing reserve1")?;

        let reserve0 = U256::from_str(reserve0_hex)?;
        let reserve1 = U256::from_str(reserve1_hex)?;

        let state = UniswapV2State::new(reserve0, reserve1);
        states.push(state);
    }

    Ok(states)
}


fn main() {
    let file_path = "./liquidity_data.json"; // Path to your JSON file

    match parse_uniswap_v2_data(file_path) {
        Ok(states) => {
            for (i, state) in states.iter().enumerate() {
                println!("Pool {}:", i + 1);
                println!("  reserve0: {}", state.reserve0);
                println!("  reserve1: {}", state.reserve1);
            }
        }
        Err(e) => eprintln!("Error parsing data: {}", e),
    }

    println!("successfully parsed");
}
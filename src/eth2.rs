extern crate reqwest;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::*;
use crate::output::Rezzy;

static LIGHTHOUSE_GIT: &str = "https://api.github.com/repos/sigp/lighthouse/releases/latest";
static PRYSM_GIT: &str = "https://api.github.com/repos/prysmaticlabs/prysm/releases/latest";
static NIMBUS_GIT: &str = "https://api.github.com/repos/status-im/nimbus-eth2/releases/latest";
static TEKU_GIT: &str = "https://api.github.com/repos/ConsenSys/teku/releases/latest";

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    id: String,
    jsonrpc: String,
    error: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
}

fn eth_req(st: &str) -> Result<reqwest::blocking::Response> {
    let req = RpcRequest {
        jsonrpc: String::from("2.0"),
        method: String::from(st),
        params: json!([]),
        id: String::from("1"),
    };

    let serialized = match serde_json::to_string(&req) {
        Ok(s) => s,
        Err(e) => {
            let msg = Rezzy{ message: format!("Error reading request: {:?}", e) };
            msg.write_red();
            String::from("")
        },
    };

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://127.0.0.1:8545")
        .header("Content-Type", "application/json")
        .body(serialized)
        .send()?;
    Ok(res)
}

fn git_req(repo: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(repo)
        .header("User-Agent", "request")
        .send()?
        .text()?;

    let j: serde_json::Value = match serde_json::from_str(res.as_str()) {
        Ok(s) => s,
        Err(e) => {
            let msg = Rezzy{ message: format!("Error reading request: {:?}", e) };
            msg.write_red();
            json![""]
        },
    };
    let mut x = "";
    match j["tag_name"].as_str() {
        Some(v) => x = v,
        None => {
            let msg = Rezzy{ message: format!("Could not pull client release version") };
            msg.write_red();
        },
    }
    Ok(String::from(x))
}

fn eth2_sync_check() -> Result<bool> {

    let client = reqwest::blocking::Client::new();
    let res = client.get("http://127.0.0.1:3500/eth/v1alpha1/node/syncing")
        .header("User-Agent", "request")
        .send()?
        .text()?;

    let j: serde_json::Value = match serde_json::from_str(res.as_str()) {
        Ok(s) => s,
        Err(e) => {
            let msg = Rezzy{ message: format!("Error reading request: {:?}", e) };
            msg.write_red();
            json![""]
        },
    };

    let mut x = true;
    match j["syncing"].as_bool() {
        Some(v) => x = v,
        None => {
            let msg = Rezzy{ message: format!("Could not get syncing status of ETH2 validator") };
            msg.write_red();
        },
    }
    Ok(x)
}

fn eth2_peer_count() -> Result<usize> {

    let client = reqwest::blocking::Client::new();
    let res = client.get("http://127.0.0.1:3500/eth/v1alpha1/node/peers")
        .header("User-Agent", "request")
        .send()?
        .text()?;

    let j: serde_json::Value = match serde_json::from_str(res.as_str()) {
        Ok(s) => s,
        Err(e) => {
            let msg = Rezzy{ message: format!("Error reading request: {:?}", e) };
            msg.write_red();
            json![""]
        },
    };

    let mut x = 0;
    match Option::Some(j["peers"].as_array().unwrap().len()) {
        Some(v) => x = v,
        None => {
            let msg = Rezzy{ message: format!("Could not get peer count of ETH2 validator") };
            msg.write_red();
        },
    }
    Ok(x)
}

pub fn eth2_check(eth2: &str) -> Result<()> {
    let banner = Rezzy{ message: format!("\nETH2 Client Check: {}", eth2) };
    banner.bold();

    let res4 = eth_req("web3_clientVersion")?;
    let r4 = res4.status();

    match r4 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res4.json()?;
            let ver = String::from(j.result.unwrap().as_str().unwrap());
            let mut repo = LIGHTHOUSE_GIT;
            match eth2 {
                "PRYSM" => repo = PRYSM_GIT,
                "NIMBUS" => repo = NIMBUS_GIT,
                "TEKU" => repo = TEKU_GIT,
                _ => (),
            }

            match git_req(repo){
                Ok(r) => {
                    if ver.contains(&r.as_str()) {
                        let msg = Rezzy{ message: format!("{}({}) is the latest release: {:?}", eth2, &ver, &r)  };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("{} needs to be updated to latest release: {}", eth2, &r) };
                        msg.write_red();
                    }
                },
                Err(e) => {
                    let msg = Rezzy{ message: format!("{} error fetching git release: {:?}", eth2, e) };
                    msg.write_red();
                }
            };
        }
        _ => {
            let msg = Rezzy{ message: format!("Could not get the latest release version from: {}", eth2) };
            msg.write_red();
        }
    }

    match eth2_sync_check(){
        Ok(r) => {
            if r {
                let msg = Rezzy{ message: format!("{} is currently synced!", eth2) };
                msg.write_green();
            }
            else {
                let msg = Rezzy{ message: format!("{} is NOT currently synced", eth2) };
                msg.write_red();
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    };
    
    match eth2_peer_count(){
        Ok(r) => {
            if r > 50 {
                let msg = Rezzy{ message: format!("{} currently has {:?} peers", eth2, r)  };
                msg.write_green();
            } else {
                let msg = Rezzy{ message: format!("{} does NOT have enough peers(Current:{})", eth2, r) };
                msg.write_red();
            }
        },
        Err(e) => {
            println!("{}", e)
        }
    };
    Ok(())
}
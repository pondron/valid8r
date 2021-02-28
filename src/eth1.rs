extern crate reqwest;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::*;
use crate::output::Rezzy;

static GETH_GIT: &str = "https://api.github.com/repos/ethereum/go-ethereum/releases/latest";
static BESU_GIT: &str = "https://api.github.com/repos/hyperledger/besu/releases/latest";
static NETHERMIND_GIT: &str = "https://api.github.com/repos/nethermindeth/nethermind/releases/latest";
static OPENETHEREUM_GIT: &str = "https://api.github.com/repos/openethereum/openethereum/releases/latest";
static INFURA: &str = "https://mainnet.infura.io/v3/65daaf22efb6473e8b56161095669ca8";

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
    let res = client.post("http://0.0.0.0:8545")
        .header("Content-Type", "application/json")
        .body(serialized)
        .send()?;
    Ok(res)
}
fn infura_req(st: &str) -> Result<reqwest::blocking::Response> {
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
    let res = client.post("https://mainnet.infura.io/v3/65daaf22efb6473e8b56161095669ca8")
        .header("Content-Type", "application/json")
        .body(serialized)
        .send()?;
    Ok(res)
}

fn git_req(mut repo: &str) -> Result<String> {
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

pub fn eth1_check(eth1: &str) -> Result<()> {
    let banner = Rezzy{ message: format!("\nETH1 Client Check: {}", eth1) };
    banner.bold();

    let res4 = eth_req("web3_clientVersion")?;
    let r4 = res4.status();

    match r4 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res4.json()?;
            let ver = String::from(j.result.unwrap().as_str().unwrap());
            let mut repo = GETH_GIT;
            match eth1 {
                "BESU" => repo = BESU_GIT,
                "NETHERMIND" => repo = NETHERMIND_GIT,
                "OPENETHEREUM" => repo = OPENETHEREUM_GIT,
                _ => (),
            }

            match git_req(repo){
                Ok(r) => {
                    if ver.contains(&r.as_str()) {
                        let msg = Rezzy{ message: format!("{}({}) is the latest release: {:?}", eth1, &ver, &r)  };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("{} needs to be updated to latest release: {}", eth1, &r) };
                        msg.write_red();
                    }
                },
                Err(e) => {
                    let msg = Rezzy{ message: format!("{} error fetching git release: {:?}", eth1, e) };
                    msg.write_red();
                }
            };
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from {}", eth1) };
            msg.write_red();
        }
    }
    let res3 = eth_req("net_version")?;
    let r3 = res3.status();

    match r3 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res3.json()?;
            if let Some(re) = j.result { 
                if let Some(st) = re.as_str() {
                    if st.eq("1") {
                        let msg = Rezzy{ message: format!("{} is on mainnet", eth1)  };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("{} is currently NOT on mainnet", eth1) };
                        msg.write_red();
                    }
                }
            }
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from {}", eth1) };
            msg.write_red();
        }
    }
    match infura_req("eth_blockNumber") {
        Ok(r) => {
            let inf: RpcResponse = r.json()?;
            if let Some(infr) = inf.result {
                if let Some(infb) = infr.as_str() {
                    let msg = Rezzy{ message: format!("Valid8r can reach Infura at latest block: {:?}(verify at https://etherscan.io/blocks)", i64::from_str_radix(infb.trim_start_matches("0x"), 16).unwrap()) };
                    msg.write_green();
                }
            }       
        },
        Err(e) => {
            let msg = Rezzy{ message: format!("VALID8R could not reach infura: {:?}", e) };
            msg.write_red();
        } 
    };

    let res1 = eth_req("eth_blockNumber")?;
    let ji: RpcResponse = res1.json()?;


    let res5 = eth_req("eth_syncing")?;
    let r5 = res5.status();

    match r5 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res5.json()?;
            match j.result {
                Some(r) => {
                    if let Some(re) = r.as_bool() {
                        if !re {
                            let msg = Rezzy{ message: format!("{} is in sync, latest block: {:?}(verify at https://etherscan.io/blocks)", eth1, i64::from_str_radix(ji.result.unwrap().as_str().unwrap().trim_start_matches("0x"), 16).unwrap())  };
                            msg.write_green();
                        }
                    } else {
                        let msg = Rezzy{ message: format!("{} is NOT currently syncd", eth1) };
                        msg.write_red();
                    }
                },
                None => {
                    let msg = Rezzy{ message: format!("{} -> VALID8R communication error", eth1) };
                    msg.write_red();
                },
            }
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get block status from {}", eth1) };
            msg.write_red();
        }
    }
    let res2 = eth_req("net_peerCount")?;
    let r2 = res2.status();

    match r2 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res2.json()?;
            match j.result {
                Some(re) => {
                    if let Some(st) = re.as_str() {
                        if let Ok(val) = i64::from_str_radix(st.trim_start_matches("0x"), 16) {
                            if val > 16 {
                                let msg = Rezzy{ message: format!("{} currently has {:?} peers", eth1, val)  };
                                msg.write_green();
                            } else {
                                let msg = Rezzy{ message: format!("{} does NOT have enough peers({})", eth1, val) };
                                msg.write_red();
                            }
                        }
                    }
                },
                None => {
                    let msg = Rezzy{ message: format!("unable to get peer count from {}", eth1) };
                    msg.write_red();
                },
            }
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from {}", eth1) };
            msg.write_red();
        }
    }
    
    Ok(())
}


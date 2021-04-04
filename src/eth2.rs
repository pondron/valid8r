#[allow(unused_imports)]
extern crate reqwest;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::*;
use anyhow::Result;
use crate::output::Rezzy;

static LIGHTHOUSE_GIT: &str = "https://api.github.com/repos/sigp/lighthouse/releases/latest";
static PRYSM_GIT: &str = "https://api.github.com/repos/prysmaticlabs/prysm/releases/latest";
static NIMBUS_GIT: &str = "https://api.github.com/repos/status-im/nimbus-eth2/releases/latest";
static TEKU_GIT: &str = "https://api.github.com/repos/ConsenSys/teku/releases/latest";

static ETH2_CLIENT_ADDR: &str = "http://127.0.0.1:5052";
static ETH2_CLIENT_ADDR_PRYSM: &str = "http://127.0.0.1:3500";
static ETH2_CLIENT_ADDR_NIMBUS: &str = "http://127.0.0.1:9190";

#[derive(Serialize, Deserialize, Debug)]
struct Eth2Response {
    data: Option<serde_json::Value>,
}

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

fn eth_rpc_req(st: &str) -> Result<reqwest::blocking::Response> {
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
    let res = client.post("http://127.0.0.1:9091")
        .header("Content-Type", "application/json")
        .body(serialized)
        .send()?;
    Ok(res)
}

fn eth2_req(endpoint: &str) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(endpoint)
        .header("Content-Type", "application/json")
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

fn eth2_sync_check(endpoint: &str) -> Result<bool> {
    let res = eth2_req(endpoint)?;
    //let res = client.get("http://127.0.0.1:3500/eth/v1alpha1/node/syncing")
    let pay: Eth2Response = res.json()?;
    let mut x = true;
    if let Some(j) = pay.data{
        match j["is_syncing"].as_bool() {
            Some(v) => {
                if v {
                    if let Some(re) = j["sync_distance"].as_str() {
                        let val: usize = re.parse()?;
                        let msg = Rezzy{ message: format!("Sync Distance: {:?}", val) };
                        msg.write_red();
                    }
                }
                x = v;
            },
            None => {
                let msg = Rezzy{ message: format!("Could not get syncing status of ETH2 validator") };
                msg.write_red();
            },
        }
    };

    Ok(x)
}

fn eth2_peer_count(endpoint: &str, eth2: &str) -> Result<usize> {
    let res = eth2_req(endpoint)?;
    let mut x = 0;

    if eth2 == "PRYSM" {
        let j = res.text()?;
        let k: serde_json::Value = match serde_json::from_str(j.as_str()) {
            Ok(s) => s,
            Err(e) => {
                let msg = Rezzy{ message: format!("Error reading request: {:?}", e) };
                msg.write_red();
                json![""]
            },
        };
        match Option::Some(k["peers"].as_array().unwrap().len()) {
            Some(v) => x = v,
            None => {
                let msg = Rezzy{ message: format!("Could not get peer count of ETH2 validator") };
                msg.write_red();
            }
        }
    } else {
        let pay: Eth2Response = res.json()?;
        if let Some(j) = pay.data {
            match j["connected"].as_str() {
                Some(v) => {
                    let val: usize = v.parse()?;
                    x = val
                },
                None => {
                    let msg = Rezzy{ message: format!("Could not get peer count of ETH2 validator") };
                    msg.write_red();
                },
            }
        }
    }
    Ok(x)
}
fn parse_ver(pay: &Eth2Response) -> Result<String> {
    let mut x = "";

    if let Some(j) = pay.data.as_ref() {
        match j["version"].as_str() {
            Some(v) => x = v,
            None => {
                let msg = Rezzy{ message: format!("Could not pull client release version") };
                msg.write_red();
            },
        }
    };

    Ok(String::from(x))
}

pub fn eth2_check(eth2: &str) -> Result<()> {
    let banner = Rezzy{ message: format!("\nETH2 Client Check: {}", eth2) };
    banner.bold();


    let base_path = match eth2 {
        "PRYSM" => ETH2_CLIENT_ADDR_PRYSM,
        "NIMBUS" => ETH2_CLIENT_ADDR_NIMBUS,
        _ => ETH2_CLIENT_ADDR,
    };


    match eth2 {
        "NIMBUS" => {
            let res4 = eth_rpc_req("getNodeVersion").unwrap();
            let r4 = res4.status();
        
            match r4 {
                reqwest::StatusCode::OK => {
                    let j: RpcResponse = res4.json()?;
                    let mut ver = String::from("");
                    if let Some(re) = j.result {
                        if let Some(v) = re.as_str() {
                            ver = String::from(v);
                        }
                    }
                    let repo = NIMBUS_GIT;

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

            let res5 = eth_rpc_req("getSyncing")?;
            let r5 = res5.status();
        
            match r5 {
                reqwest::StatusCode::OK => {
                    let j: RpcResponse = res5.json()?;
                    match j.result {
                        Some(r) => {
                            if let Some(re) = r.as_bool() {
                                if !re {
                                    let msg = Rezzy{ message: format!("{} is in sync (verify at https://etherscan.io/blocks)", eth2)};
                                    msg.write_green();
                                }
                            } else {
                                let msg = Rezzy{ message: format!("{} is NOT currently synced", eth2) };
                                msg.write_red();
                            }
                        },
                        None => {
                            let msg = Rezzy{ message: format!("{} -> VALID8R communication error", eth2) };
                            msg.write_red();
                        },
                    }
                }
                _ => {
                    let msg = Rezzy{ message: format!("unable to get block status from {}", eth2) };
                    msg.write_red();
                }
            }

            let res2 = eth_rpc_req("get_v1_node_peer_count")?;
            let r2 = res2.status();
        
            match r2 {
                reqwest::StatusCode::OK => {
                    let j: RpcResponse = res2.json()?;
                    let j_as_ref = j.result;

                    if let Some(j) = j_as_ref.as_ref() {
                        match j["connected"].as_i64() {
                            Some(re) => {
                                if re > 10 {
                                    let msg = Rezzy{ message: format!("{} currently has {:?} peers", eth2, re)  };
                                    msg.write_green();
                                } else {
                                    let msg = Rezzy{ message: format!("{} has low peer count: peers(Current:{})", eth2, re) };
                                    msg.write_yellow();
                                }
                            },                            
                            None => {
                                let msg = Rezzy{ message: format!("unable to get peer count from {}", eth2) };
                                msg.write_red();
                            },
                        }
                    }
                }
                _ => {
                    let msg = Rezzy{ message: format!("unable to get peer count from {}", eth2) };
                    msg.write_red();
                }
            }
        }
        _ => {
            let mut version = "";
            if eth2 == "PRYSM" {
                version = "v1alpha1"
            } else {
                version = "v1"
            }
            let res = eth2_req(format!("{}/eth/{}/node/version", base_path, version).as_str())?;
            let r = res.status();
        
            match r {
                reqwest::StatusCode::OK => {
                    let j: Eth2Response = res.json()?;
                    let ver = parse_ver(&j)?;
        
                    let mut repo = LIGHTHOUSE_GIT;
                    match eth2 {
                        "PRYSM" => repo = PRYSM_GIT,
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

            match eth2_sync_check(format!("{}/eth/{}/node/syncing", base_path, version).as_str()) {
                Ok(r) => {
                    if !r {
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
            
            let mut peers = "";
            if eth2 == "PRYSM" {
                peers = "peers"
            } else {
                peers = "peer_count"
            }

            match eth2_peer_count(format!("{}/eth/{}/node/{}", base_path, version, peers).as_str(), eth2){
                Ok(r) => {
                    if r > 10 {
                        let msg = Rezzy{ message: format!("{} currently has {:?} peers", eth2, r)  };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("{} has low peer count: peers(Current:{})", eth2, r) };
                        msg.write_yellow();
                    }
                },
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    };
    Ok(())
}
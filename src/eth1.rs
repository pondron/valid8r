extern crate reqwest;
use serde::{Serialize, Deserialize};
use serde_json::json;
use anyhow::Result;
use crate::output::Rezzy;

static GETH_GIT: &str = "https://api.github.com/repos/ethereum/go-ethereum/releases/latest";
static BESU_GIT: &str = "https://api.github.com/repos/hyperledger/besu/releases/latest";
static NETHERMIND_GIT: &str = "https://api.github.com/repos/nethermindeth/nethermind/releases/latest";
static OPENETHEREUM_GIT: &str = "https://api.github.com/repos/openethereum/openethereum/releases/latest";

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub id: String,
    pub jsonrpc: String,
    pub error: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq)]
pub struct Eth1Client {
    pub name: String,
    pub http_addr: String,
    pub infura_addr: String,
    pub testnet: bool,
}

impl Eth1Client {
    pub fn new(name: String, http_addr: String, infura_addr: String, testnet: bool) -> Eth1Client {
        Eth1Client{
            name,
            http_addr,
            infura_addr,
            testnet,
        }
    }
    pub fn infura_req(&self, st: &str) -> Result<reqwest::blocking::Response> {
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
        let res = client.post(self.http_addr.as_str())
            .header("Content-Type", "application/json")
            .body(serialized)
            .send()?;
        Ok(res)
    }
    pub fn eth1_check(&self) -> Result<()> {
        let banner = Rezzy{ message: format!("\nETH1 Client Check: {}", self.name) };
        banner.bold();
    
        let res4 = eth_req("web3_clientVersion", self.http_addr.as_str())?;
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
                let mut repo = GETH_GIT;
                match self.name.as_str() {
                    "BESU" => repo = BESU_GIT,
                    "NETHERMIND" => repo = NETHERMIND_GIT,
                    "OPENETHEREUM" => repo = OPENETHEREUM_GIT,
                    _ => (),
                }
    
                match git_req(repo){
                    Ok(r) => {
                        if ver.contains(&r.as_str()) {
                            let msg = Rezzy{ message: format!("{}({}) is the latest release: {:?}", self.name, &ver, &r)  };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{} needs to be updated to latest release: {}", self.name, &r) };
                            msg.write_red();
                        }
                    },
                    Err(e) => {
                        let msg = Rezzy{ message: format!("{} error fetching git release: {:?}", self.name, e) };
                        msg.write_red();
                    }
                };
            }
            _ => {
                let msg = Rezzy{ message: format!("Could not get the latest release version from: {}", self.name) };
                msg.write_red();
            }
        }
        let res3 = eth_req("net_version", self.http_addr.as_str())?;
        let r3 = res3.status();
    
        match r3 {
            reqwest::StatusCode::OK => {
                let j: RpcResponse = res3.json()?;
                if let Some(re) = j.result { 
                    if let Some(st) = re.as_str() {
                        if st.eq("1") && !self.testnet {
                            let msg = Rezzy{ message: format!("{} is on mainnet", self.name)  };
                            msg.write_green();
                        } else if !st.eq("1") && self.testnet {
                            let msg = Rezzy{ message: format!("{} is on testnet: {}", self.name, st)  };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{} is currently NOT on mainnet", self.name) };
                            msg.write_red();
                        }
                    }
                }
            }
            _ => {
                let msg = Rezzy{ message: format!("Unable to get environment from {}", self.name) };
                msg.write_red();
            }
        }
        match self.infura_req("eth_blockNumber") {
            Ok(r) => {
                let inf: RpcResponse = r.json()?;
                if let Some(infr) = inf.result {
                    if let Some(infb) = infr.as_str() {
                        let msg = Rezzy{ message: format!("Valid8r can reach Infura at latest block: {:?}(verify at https://etherscan.io/blocks)", i64::from_str_radix(infb.trim_start_matches("0x"), 16)?) };
                        msg.write_green();
                    }
                }       
            },
            Err(e) => {
                let msg = Rezzy{ message: format!("VALID8R could not reach infura: {:?}", e) };
                msg.write_red();
            } 
        };
    
        let res1 = eth_req("eth_blockNumber", self.http_addr.as_str())?;
        let ji: RpcResponse = res1.json()?;
    
    
        let res5 = eth_req("eth_syncing", self.http_addr.as_str())?;
        let r5 = res5.status();
    
        match r5 {
            reqwest::StatusCode::OK => {
                let j: RpcResponse = res5.json()?;
                match j.result {
                    Some(r) => {
                        if let Some(re) = r.as_bool() {
                            if !re {
                                if let Some(re) = ji.result {
                                    if let Some(val) = re.as_str() {
                                        let msg = Rezzy{ message: format!("{} is in sync, latest block: {:?}(verify at https://etherscan.io/blocks)", self.name, i64::from_str_radix(val.trim_start_matches("0x"), 16)?)  };
                                        msg.write_green();
                                    }
                                } else {
                                    let msg = Rezzy{ message: format!("Could not parse sync data") };
                                    msg.write_red();
                                }
                            }
                        } else {
                            if let Ok(val) = i64::from_str_radix(r["currentBlock"].as_str().unwrap().trim_start_matches("0x"), 16) {
                                let msg = Rezzy{ message: format!("{} is NOT currently synced: {:?}", self.name, val) };
                                msg.write_red();
                            }
                        }
                    },
                    None => {
                        let msg = Rezzy{ message: format!("{} -> VALID8R communication error", self.name) };
                        msg.write_red();
                    },
                }
            }
            _ => {
                let msg = Rezzy{ message: format!("unable to get block status from {}", self.name) };
                msg.write_red();
            }
        }
        let res2 = eth_req("net_peerCount", self.http_addr.as_str())?;
        let r2 = res2.status();
    
        match r2 {
            reqwest::StatusCode::OK => {
                let j: RpcResponse = res2.json()?;
                match j.result {
                    Some(re) => {
                        if let Some(st) = re.as_str() {
                            if let Ok(val) = i64::from_str_radix(st.trim_start_matches("0x"), 16) {
                                if val > 16 {
                                    let msg = Rezzy{ message: format!("{} currently has {:?} peers", self.name, val)  };
                                    msg.write_green();
                                } else {
                                    let msg = Rezzy{ message: format!("{} has low peer count: peers(Current:{})", self.name, val) };
                                    msg.write_yellow();
                                }
                            }
                        }
                    },
                    None => {
                        let msg = Rezzy{ message: format!("unable to get peer count from {}", self.name) };
                        msg.write_red();
                    },
                }
            }
            _ => {
                let msg = Rezzy{ message: format!("unable to get peer count from {}", self.name) };
                msg.write_red();
            }
        }
        
        Ok(())
    }
}

pub fn eth_req(st: &str, url: &str) -> Result<reqwest::blocking::Response> {
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
    let res = client.post(url)
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




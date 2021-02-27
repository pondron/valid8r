extern crate reqwest;
use serde::{Serialize, Deserialize};
use serde_json::json;
use reqwest::*;
use crate::output::Rezzy;
use std::collections::BTreeMap as Map;

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

    let serialized = serde_json::to_string(&req).unwrap();

    let client = reqwest::blocking::Client::new();
    let res = client.post("http://0.0.0.0:8545")
        .header("Content-Type", "application/json")
        .body(serialized)
        .send()?;
    Ok(res)
}

//GIT TAG: String("v1.9.25")
//ETH1 Client Check: GETH(Geth/v1.9.25-stable-e7872729/linux-amd64/go1.15.6)
//GETH is in sync: Some(Bool(false))
//GETH has found peers: Some(String("0x32"))
//GETH is on mainnet: Some(String("1"))

fn git_req() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://api.github.com/repos/ethereum/go-ethereum/releases/latest")
        .header("User-Agent", "request")
        .send().unwrap()
        .text().unwrap();

    println!("JSON: {:?}", res);
    let j: serde_json::Value = serde_json::from_str(res.as_str()).unwrap();
    println!("GIT TAG: {:?}", j["tag_name"]);
    Ok(())
}

pub fn eth1_check(eth1: &str) -> Result<()> {
    let n = git_req();
    let res4 = eth_req("web3_clientVersion").unwrap();
    let r4 = res4.status();

    match r4 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res4.json().unwrap();
            let banner = Rezzy{ message: format!("\nETH1 Client Check: {}({})", eth1, j.result.unwrap().as_str().unwrap()) };
            banner.bold();
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from GETH") };
            msg.write_red()
        }
    }

    let res1 = eth_req("eth_syncing").unwrap();
    let r1 = res1.status();

    match r1 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res1.json().unwrap();
            let msg = Rezzy{ message: format!("{} is in sync: {:?}", eth1, j.result)  };
            msg.write_green();
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from GETH") };
            msg.write_red()
        }
    }
    let res2 = eth_req("net_peerCount").unwrap();
    let r2 = res2.status();

    match r2 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res2.json().unwrap();
            let msg = Rezzy{ message: format!("{} has found peers: {:?}", eth1, j.result)  };
            msg.write_green();
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from GETH") };
            msg.write_red()
        }
    }
    let res3 = eth_req("net_version").unwrap();
    let r3 = res3.status();

    match r3 {
        reqwest::StatusCode::OK => {
            let j: RpcResponse = res3.json().unwrap();
            let msg = Rezzy{ message: format!("{} is on mainnet: {:?}", eth1, j.result)  };
            msg.write_green();
        }
        _ => {
            let msg = Rezzy{ message: format!("unable to get peer count from GETH") };
            msg.write_red()
        }
    }
    
    Ok(())
}

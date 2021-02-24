use serde::{Serialize, Deserialize};
use serde_json::json;
// use crate::output::Rezzy;

#[derive(Serialize, Deserialize, Debug)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: String,
}

pub fn geth_check() {

    let req = RpcRequest {
        jsonrpc: String::from("2.0"),
        method: String::from("eth_getBalance"),
        params: json!(["0xF107A9b3A91d0Fe0e063e37e4dD3F6fd2dC3bdC6", "latest"]),
        id: String::from("1"),
    };

    let serialized = serde_json::to_string(&req).unwrap();
    println!("serialized = {}", serialized);

    // let client = reqwest::Client::new();
    // let res = client.post("http://0.0.0.0:8545")
    //     .json(&serialized)
    //     .send()
    //     .await?;

    // printlin!(res);

    // let deserialized: RpcRequest = serde_json::from_str(&serialized).unwrap();
    // println!("deserialized = {:?}", deserialized);
}
pub fn besu_check() {

}
pub fn nethermind_check() {

}
pub fn open_ethereum_check() {

}
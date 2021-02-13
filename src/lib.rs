use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    // ethereum 1.0 client 
    #[structopt(short = "1", long)]
    pub eth1: String,

    // ethereum 2.0 client 
    #[structopt(short = "2", long)]
    pub eth2: String,

    // optional: path to custom config file
    // #[structopt(parse(from_os_str))]
    // pub file: std::path::PathBuf,
}

#[derive(Debug,PartialEq)]
pub enum Eth1Client {
    GETH,
    BESU,
    NETHERMIND,
    OPENETHEREUM,
    NONE,
}

#[derive(Debug,PartialEq)]
pub enum Eth2Client {
    LIGHTHOUSE,
    PRYSM,
    TEKU,
    NIMBUS,
    NONE,
}

#[derive(Debug,PartialEq)]
pub struct Valid8r {
    pub eth1: Eth1Client,
    pub eth2: Eth2Client,
}

impl Valid8r {
    pub fn new(cfg: Config) -> Valid8r {
        let mut v = Valid8r{ eth1: Eth1Client::NONE, eth2: Eth2Client::NONE };

        let e1: &str = &cfg.eth1.to_lowercase();
        match e1 {
            "geth" => v.eth1 = Eth1Client::GETH,
            "besu" => v.eth1 = Eth1Client::BESU,
            "nethermind" => v.eth1 = Eth1Client::NETHERMIND,
            "openethereum" => v.eth1 = Eth1Client::OPENETHEREUM,
            _ => println!("BAD BAD BAD"),
        }

        let e2: &str = &cfg.eth2.to_lowercase();
        match e2 {
            "lighthouse" => v.eth2 = Eth2Client::LIGHTHOUSE,
            "prysm" => v.eth2 = Eth2Client::PRYSM,
            "teku" => v.eth2 = Eth2Client::TEKU,
            "nimbus" => v.eth2 = Eth2Client::NIMBUS,
            _ => println!("BAD BAD BAD"),
        }

        v
    }
}

pub fn run(valid: Valid8r) -> Result<(), Box<dyn Error>> {
    // todo: begin concurrency and client based checks here 
    println!("1: {:?}", valid.eth1);
    println!("2: {:?}", valid.eth2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_arg_match() {
        let cfg = Config{ eth1: String::from("GETH"), eth2: String::from("LIGHTHOUSE") };
        let val = Valid8r::new(cfg);
        assert_eq!(val.eth1, Eth1Client::GETH);
        assert_eq!(val.eth2, Eth2Client::LIGHTHOUSE);
    }
}
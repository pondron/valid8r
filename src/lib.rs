use sysinfo::{System, SystemExt, DiskExt};
use std::net::TcpListener;
use std::io::ErrorKind;
use std::process;
use std::error::{Error as Err};
use structopt::StructOpt;
use chrono::prelude::*;
use output::Rezzy;
use eth2::*;

mod output;
mod eth1;
mod eth2;

#[derive(StructOpt)]
pub struct Config {
    // ethereum 1.0 client 
    #[structopt(short = "1", long)]
    pub eth1: String,

    // optional: ethereum 1.0 client listener port
    #[structopt(long)]
    pub eth1_listener_port: Option<i32>,

    // optional: ethereum 1.0 client http port
    #[structopt(long)]
    pub eth1_http_port: Option<i32>,

    // ethereum 2.0 client 
    #[structopt(short = "2", long)]
    pub eth2: String,

    // optional: ethereum 2.0 client listener port
    #[structopt(long)]
    pub eth2_listener_port: Option<i32>,

    // optional: ethereum 2.0 client http port
    #[structopt(long)]
    pub eth2_http_port: Option<i32>,

    // optional: testnet bool flag
    #[structopt(short = "t", long)]
    pub testnet: Option<String>,

    // optional: ntp endpoint 
    #[structopt(short = "n", long)]
    pub ntp_endpoint: Option<String>,

    // optional: infura endpoint
    #[structopt(short = "i", long)]
    pub infura_endpoint: Option<String>,
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
    pub eth1: eth1::Eth1Client,
    pub eth1_listener_addr: String,
    pub eth1_http_addr: String,
    pub eth2: Eth2Client,
    pub eth2_listener_addr: String,
    pub eth2_http_addr: String,
    pub ntp_endpoint: String,
}

impl Valid8r {
    pub fn new(cfg: Config) -> Valid8r {
        let mut v = Valid8r{
            eth1: eth1::Eth1Client::new(
                String::from("GETH"),
                String::from("http://127.0.0.1:8545"),
                String::from("https://mainnet.infura.io/v3/65daaf22efb6473e8b56161095669ca8"),
                false)
            ,
            eth1_listener_addr: String::from("0.0.0.0:30303"),
            eth1_http_addr: String::from("127.0.0.1:8545"),
            eth2: Eth2Client::NONE,
            eth2_listener_addr: String::from("0.0.0.0:9000"),
            eth2_http_addr: String::from("0.0.0.0:5052"),
            ntp_endpoint: String::from("0.pool.ntp.org:123"),
        };

        if let Some(ntp) = cfg.ntp_endpoint {
            v.ntp_endpoint = ntp;
        }        
        if let Some(infura) = cfg.infura_endpoint {
            v.eth1.infura_addr = infura;
        }
        if let Some(_) = cfg.testnet {
            v.eth1.testnet = true;
        }

        let e1: &str = &cfg.eth1.to_lowercase();
        match e1 {
            "geth" => v.eth1.name = String::from("GETH"),
            "besu" => v.eth1.name = String::from("BESU"),
            "nethermind" => v.eth1.name = String::from("NETHERMIND"),
            "openethereum" => v.eth1.name = String::from("OPENETHEREUM"),
            _ => {
                println!("ERROR: Please input a valid Eth1 client(entered {})", e1);
                process::exit(1);
            },
        }

        let e2: &str = &cfg.eth2.to_lowercase();
        match e2 {
            "lighthouse" => v.eth2 = Eth2Client::LIGHTHOUSE,
            "prysm" => {
                v.eth2 = Eth2Client::PRYSM;
                v.eth2_listener_addr = String::from("0.0.0.0:4000");
                v.eth2_http_addr = String::from("127.0.0.1:3500");
            },
            "teku" => v.eth2 = Eth2Client::TEKU,
            "nimbus" => {
                v.eth2 = Eth2Client::NIMBUS;
                v.eth2_http_addr = String::from("127.0.0.1:9091");
            },
            _ => {
                println!("ERROR: Please input a valid Eth2 client(entered {})", e1);
                process::exit(1);
            },
        }

        if let Some(port) = cfg.eth1_listener_port {
            v.eth1_listener_addr = format!("0.0.0.0:{}", port);
        }
        if let Some(port) = cfg.eth1_http_port {
            v.eth1_http_addr = format!("127.0.0.1:{}", port);
            v.eth1.http_addr = format!("http://127.0.0.1:{}", port);
        }
        if let Some(port) = cfg.eth2_listener_port {
            v.eth2_listener_addr = format!("0.0.0.0:{}", port);
        }
        if let Some(port) = cfg.eth1_http_port {
            v.eth2_http_addr = format!("127.0.0.1:{}", port);
        }


        v
    }
    pub fn run(&self) -> Result<(), Box<dyn Err>> {    
        let banner = Rezzy{ message: format!("Valid8r is Valid8ing your Valid8r") };
        banner.bold();

        self.sys_req();

        self.net_req();

        if let Err(_e) = self.eth1.eth1_check() {
            let msg = Rezzy{ message: format!("VALID8R could not connect to {} at addr {}", self.eth1.name, self.eth1.http_addr) };
            msg.write_red();
        }

        match self.eth2 {
            Eth2Client::LIGHTHOUSE => {
                if let Err(_e) = eth2_check("LIGHTHOUSE", format!("http://{}", self.eth2_http_addr)) {
                    let msg = Rezzy{ message: format!("VALID8R could not connect to LIGHTHOUSE") };
                    msg.write_red();
                }
            }
            Eth2Client::PRYSM => {
                if let Err(_e) = eth2_check("PRYSM", format!("http://{}", self.eth2_http_addr)) {
                    let msg = Rezzy{ message: format!("VALID8R ERROR could not connect to PRYSM") };
                    msg.write_red();
                }
            },
            Eth2Client::NIMBUS => {
                if let Err(_e) = eth2_check("NIMBUS", format!("http://{}", self.eth2_http_addr)) {
                    let msg = Rezzy{ message: format!("VALID8R ERROR could not connect to NIMBUS") };
                    msg.write_red();
                }
            },
            Eth2Client::TEKU => {
                if let Err(_e) = eth2_check("TEKU",format!("http://{}", self.eth2_http_addr)) {
                    let msg = Rezzy{ message: format!("VALID8R ERROR could not connect to TEKU") };
                    msg.write_red();
                }
            },
            _ => println!("can't happen")
        }

        println!("\n");
        
        Ok(())
    }
    pub fn net_req(&self) {
        let banner = Rezzy{ message: format!("\nNetwork Requirements:") };
        banner.bold();
        match self.eth1 {
            _ => {
                match TcpListener::bind(&self.eth1_listener_addr) {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{} IS NOT LISTENING ON PORT: {}", self.eth1.name, self.eth1_listener_addr) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{} is listening on port: {}", self.eth1.name, self.eth1_listener_addr) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on {}", e, self.eth1_listener_addr) };
                            msg.write_yellow();
                        }
                    }
                }
                match TcpListener::bind(&self.eth1_http_addr) {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{} IS NOT LISTENING for JSON RPC on PORT: {}", self.eth1.name, self.eth1_http_addr) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{} is listening on port: {}", self.eth1.name, self.eth1_http_addr) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on {}", e, self.eth1_http_addr) };
                            msg.write_yellow();
                        }
                    }
                }
            }
        }
        match self.eth2 {
            Eth2Client::LIGHTHOUSE | Eth2Client::NIMBUS | Eth2Client::TEKU => {
                match TcpListener::bind(&self.eth2_listener_addr) {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{:?} IS NOT LISTENING ON PORT: 9000", self.eth2) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{:?} is listening on port: 9000", self.eth2) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on 9000", e) };
                            msg.write_yellow();
                        }
                    }
                }   
            }
            Eth2Client::PRYSM => {
                match TcpListener::bind(&self.eth2_listener_addr) {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{:?} IS NOT LISTENING ON PORT: 4000", self.eth2) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{:?} is listening on port: 4000", self.eth2) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on 4000", e) };
                            msg.write_yellow();
                        }
                    }
                }   
            } 
            _ => {
                // figure out a better way of handling this,
            }
        }
        match TcpListener::bind("127.0.0.1:22") {
            Ok(_) => {
                let msg = Rezzy{ message: format!("No default ssh agent running on port: 22") };
                msg.write_green();
            },
            Err(e) => {
                if e.kind() == ErrorKind::AddrInUse {
                    let msg = Rezzy{ message: format!("{:?} security best practices recommend moving the standard ssh port", self.eth1) };
                    msg.write_red();
                } else if e.kind()  == ErrorKind::PermissionDenied {
                    let msg = Rezzy{ message: format!("Could not access default ssh port 22(run as root)") };
                    msg.write_yellow();
                } else {
                    let msg = Rezzy{ message: format!("{:?} misc error when listening on 22", e) };
                    msg.write_yellow();
                }
            }
        }
    }
    pub fn sys_req(&self) {
        let banner = Rezzy{ message: format!("\nSystem Requirements:") };
        banner.bold();
        match ntp::request(&self.ntp_endpoint) {
            Ok(val) => {
                let ntp_time = val.transmit_time;
                let loc = Local::now();
                println!("Time Sync - NTP {} vs LOCAL {:?}", ntp_time, loc.time());
            },
            Err(_) => {
                let msg = Rezzy{ message: format!("Could not get NTP time") };
                msg.write_red();
            },
        };


        let sys = System::new_all();
    
        let os = match sys.get_name(){
            Some(val) => val.to_lowercase(),
            None => return,
        };
        // check os ver
        if os.eq("ubuntu") {
            let lts = "20.04";
            match sys.get_os_version() {
                Some(cur) => {
                    if cur.eq(lts) {
                        let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("OS Version NOT up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                        msg.write_red();
                    }
                },
                None => {
                    let msg = Rezzy{ message: format!("Could not get OS Version") };
                    msg.write_red();
                },
            };
        } else if os.eq("darwin") {
            let lts = "11.2.1";
            match sys.get_os_version() {
                Some(cur) => {
                    if cur.eq("11.2.1") {
                        let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                        msg.write_green();
                    } else {
                        let msg = Rezzy{ message: format!("OS Version NOT up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                        msg.write_red();
                    }
                },
                None => {
                    let msg = Rezzy{ message: format!("Could not get OS Version") };
                    msg.write_red(); 
                }
            };
        }
    
        // check sys memory
        let mem = sys.get_total_memory();
        if mem > 16000000 {
            let msg = Rezzy{ message: format!("Memory requirement reached: \n\t Preferred 16GB(min 8GB) => Have {} KB", mem) };
            msg.write_green();
        } else if mem < 16000000 && mem > 8000000 {
            let msg = Rezzy{ message: format!("Min Memory requirement reached: \n\t Preferred 16GB(min 8GB) => Have {} KB", mem) };
            msg.write_yellow();
        } else {
            let msg = Rezzy{ message: format!("Memory requirement NOT reached: \n\t Preferred 16GB(min 8GB) => Have {} KB", mem) };
            msg.write_red();
        }
    
        // check num processors
        let proc = sys.get_processors().len();
        if proc >= 4 {
            let msg = Rezzy{ message: format!("Processor count requirement reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_green();
        } else if proc < 4 && proc > 2 {
            let msg = Rezzy{ message: format!("Min Processor count requirement reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_yellow();
        } else {
            let msg = Rezzy{ message: format!("Processor count requirement NOT reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_red();
        }
    
        let mut largest_disk = 0;
        for disk in sys.get_disks() {
            if disk.get_total_space() > largest_disk {
                largest_disk = disk.get_total_space();
            }
        }
        // check disk size requirements
        if largest_disk > 1000000000000 {
            let msg = Rezzy{ message: format!("Disk size requirement reached: \n\t Preferred 1TB(min 300GB) => Have {:?} bytes", largest_disk) };
            msg.write_green();
        } else if largest_disk < 1000000000000 && largest_disk > 300000000000 {
            let msg = Rezzy{ message: format!("Min Disk size requirement reached: \n\t Preferred 1TB(min 300GB) => Have {:?} bytes", largest_disk) };
            msg.write_yellow();
        } else {
            let msg = Rezzy{ message: format!("Disk size requirement  NOTreached: \n\t Preferred 1TB(min 300GB) => Have {:?} bytes", largest_disk) };
            msg.write_red();
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_arg_match() {
        let cfg = Config{ 
            eth1: String::from("GETH"),
            eth2: String::from("LIGHTHOUSE"),
            eth1_listener_port: Some(30303),
            eth1_http_port: Some(8545),
            eth2_listener_port: Some(9000),
            eth2_http_port: Some(5052),
            testnet: Some(String::from("Ropsten")),
            ntp_endpoint: Some(String::from("0.0.0.0")),
            infura_endpoint: Some(String::from("0.0.0.0")),
        };
        let val = Valid8r::new(cfg);
        assert_eq!(val.eth1, String::from("GETH"));
        assert_eq!(val.eth2, Eth2Client::LIGHTHOUSE);
    }
}

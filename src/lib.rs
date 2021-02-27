use sysinfo::{System, SystemExt, DiskExt};
use std::net::TcpListener;
use std::io::ErrorKind;
use std::error::{Error as Err};
use structopt::StructOpt;
use chrono::prelude::*;
use output::Rezzy;
use eth1::*;

mod output;
mod eth1;

#[derive(StructOpt)]
pub struct Config {
    // ethereum 1.0 client 
    #[structopt(short = "1", long)]
    pub eth1: String,

    // ethereum 2.0 client 
    #[structopt(short = "2", long)]
    pub eth2: String,
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
            _ => panic!("Please input a valid Eth1 client"),
        }

        let e2: &str = &cfg.eth2.to_lowercase();
        match e2 {
            "lighthouse" => v.eth2 = Eth2Client::LIGHTHOUSE,
            "prysm" => v.eth2 = Eth2Client::PRYSM,
            "teku" => v.eth2 = Eth2Client::TEKU,
            "nimbus" => v.eth2 = Eth2Client::NIMBUS,
            _ => panic!("Please input a valid Eth2 client"),
        }

        v
    }
    pub fn run(&self) -> Result<(), Box<dyn Err>> {
        // TODO: 
        //  - remove all unwraps and provide helpful errors
    
        let banner = Rezzy{ message: format!("Valid8r is Valid8ing your Valid8r") };
        banner.bold();
        self.sys_req();

        self.net_req();

        // can we talk to infura
        // are we synced w/ the latest block
        match self.eth1 {
            Eth1Client::GETH => {
                if let Err(_e) = eth1_check("GETH") {
                    let msg = Rezzy{ message: format!("VALID8R could not connect to GETH") };
                    msg.write_red();
                }
            }
            Eth1Client::BESU => {
                if let Err(_e) = eth1_check("BESU") {
                    let msg = Rezzy{ message: format!("VALID8R ERROR cound not connect to BESU") };
                    msg.write_red();
                }
            },
            Eth1Client::NETHERMIND => {
                if let Err(_e) = eth1_check("NETHERMIND") {
                    let msg = Rezzy{ message: format!("VALID8R ERROR cound not connect to NETHERMIND") };
                    msg.write_red();
                }
            },
            Eth1Client::OPENETHEREUM => {
                if let Err(_e) = eth1_check("OPENETHEREUM") {
                    let msg = Rezzy{ message: format!("VALID8R ERROR cound not connect to OPENETHEREUM") };
                    msg.write_red();
                }
            },
            _ => println!("can't happen")
        }

        //println!("\nETH2 Requirements: {:?}", self.eth2);

        Ok(())
    }
    pub fn net_req(&self) {
        let banner = Rezzy{ message: format!("\nNetwork Requirements:") };
        banner.bold();
        match self.eth1 {
            _ => {
                match TcpListener::bind("127.0.0.1:30303") {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{:?} IS NOT LISTENING ON PORT: 30303", self.eth1) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{:?} is listening on port: 30303", self.eth1) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on 30303", e) };
                            msg.write_yellow();
                        }
                    }
                }
                match TcpListener::bind("127.0.0.1:8545") {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{:?} IS NOT LISTENING ON PORT: 8545", self.eth1) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{:?} is listening on port: 8545", self.eth1) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on 8545", e) };
                            msg.write_yellow();
                        }
                    }
                }
            }
        }
        match self.eth2 {
            Eth2Client::LIGHTHOUSE | Eth2Client::NIMBUS | Eth2Client::TEKU => {
                match TcpListener::bind("127.0.0.1:9000") {
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
                match TcpListener::bind("127.0.0.1:13000") {
                    Ok(_) => {
                        let msg = Rezzy{ message: format!("{:?} IS NOT LISTENING ON PORT: 13000", self.eth2) };
                        msg.write_red();
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            let msg = Rezzy{ message: format!("{:?} is listening on port: 13000", self.eth2) };
                            msg.write_green();
                        } else {
                            let msg = Rezzy{ message: format!("{:?} misc error when listening on 13000", e) };
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
                    let msg = Rezzy{ message: format!("Could not access privilaged port 22. Either run me as root user or run `sudo netstat -lpnut | grep ssh` to ensure ssh is not running on the standard port") };
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
        let response: ntp::packet::Packet = ntp::request("0.pool.ntp.org:123").unwrap();
        let ntp_time = response.transmit_time;
        let loc = Local::now();
        println!("Time Sync - NTP {} vs LOCAL {:?}", ntp_time, loc.time());

        let sys = System::new_all();
    
        let os = sys.get_name().unwrap().to_lowercase();
        // check os ver
        if os.eq("ubuntu") {
            let lts = "20.04";
            let cur = sys.get_os_version().unwrap();
            if cur.eq(lts) {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_green();
            } else {
                let msg = Rezzy{ message: format!("OS Version NOT up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_red();
            }
        } else if os.eq("darwin") {
            let lts = "11.2.1";
            let cur = sys.get_os_version().unwrap();
            if cur.eq("11.2.1") {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_green();
            } else {
                let msg = Rezzy{ message: format!("OS Version NOT up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_red();
            }
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
        if proc > 4 {
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
            let msg = Rezzy{ message: format!("Disk size requirement reached: \n\t Preffered 1TB(min 128GB) => Have {:?} bytes", largest_disk) };
            msg.write_green();
        } else if largest_disk < 1000000000000 && largest_disk > 128000000000 {
            let msg = Rezzy{ message: format!("Min Disk size requirement reached: \n\t Preffered 1TB(min 128GB) => Have {:?} bytes", largest_disk) };
            msg.write_yellow();
        } else {
            let msg = Rezzy{ message: format!("Disk size requirement  NOTreached: \n\t Preffered 1TB(min 128GB) => Have {:?} bytes", largest_disk) };
            msg.write_red();
        }


    }    
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

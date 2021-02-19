use sysinfo::{NetworksExt, System, SystemExt, ProcessExt, DiskExt};
use std::net::TcpListener;
use std::io::{Error, ErrorKind};
use std::error::{Error as Err};
use structopt::StructOpt;
use output::Rezzy;

mod output;

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

pub const GREEN: char = '\u{2705}';
pub const YELLOW: char = '\u{26A0}';
pub const RED: char = '\u{274C}';

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
        // todo: begin concurrency and client based checks here
        //  - system checks(i.e. os up to date, sufficient hardware)
        //  - check process is running
        //  - listening port checks
        //  - check most recent block
        //  - check time sync
    
        //  - optional: check graphana up
        self.sys_req();
    
        self.net_req();
        println!("done with net\n\n");
    
        Ok(())
    }
    pub fn net_req(&self) {
        let s = System::new_all();
        for net in s.get_networks() {
            println!("{:?}", net);
        }
        match self.eth1 {
            Eth1Client::GETH => {
                match TcpListener::bind("127.0.0.1:30303") {
                    Ok(_) => println!("should not be able to do this"),
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            println!("ERROR: {}", e);
                        } else {
                            println!("different err");
                        }
                    }
                }
                match TcpListener::bind("127.0.0.1:8545") {
                    Ok(_) => println!("should not be able to do this"),
                    Err(e) => {
                        if e.kind() == ErrorKind::AddrInUse {
                            println!("ERROR: {}", e);
                        } else {
                            println!("different err");
                        }
                    }
                }
            }
            _ => println!("all eth1 on same ports"),
        }
    

    
    }
    pub fn sys_req(&self) {
        let mut sys = System::new_all();
    
        // check os ver
        if sys.get_name().unwrap().eq("Ubuntu") {
            if sys.get_os_version().unwrap().eq("20.04") {
                let msg = format!("OS Ver OKAY: {:?}", sys.get_os_version());
                let r = Rezzy::new(GREEN, msg);
                r.build_output();

            } else {
                let msg = format!("Please upgrade your OS");
                let r = Rezzy::new(RED, msg);
                r.build_output();
            }
        }
    
        // check sys memory
        let mem = sys.get_total_memory();
        if mem > 17179869 {
            let msg = format!("Memory requirement reached: {}", mem);
            let r = Rezzy::new(GREEN, msg);
            r.build_output();
        } else if mem < 17179869 && mem > 8589934 {
            let msg = format!("Minimum memory requirement reached. Current:{} Preferred:{}", mem, 17179869);
            let r = Rezzy::new(YELLOW, msg);
            r.build_output();
        } else {
            let msg= format!("Memory requirement not reached! Current:{} Required:{}", mem, 8589934);
            let r = Rezzy::new(RED, msg);
            r.build_output();
        }
    
        // check num processors
        let proc = sys.get_processors().len();
        if proc > 4 {
            let msg = format!("Processor count requirement reached: {}", proc);
            let r = Rezzy::new(GREEN, msg);
            r.build_output();
        } else if proc < 4 && proc > 2 {
            let msg = format!("Minimum processor count requirement reached. Current:{} Preferred:{}", proc, 4);
            let r = Rezzy::new(YELLOW, msg);
            r.build_output();
        } else {
            let msg = format!("Minimum processor count requirement failed. Current:{} Required:{}", proc, 4);
            let r = Rezzy::new(RED, msg);
            r.build_output();
        }
    
        // check disk size
        for disk in sys.get_disks() {
            let d = disk.get_total_space();
            if d > 1073741824000 {
                let msg = format!("Disk size requirement reached on {:?} Size:{}", disk.get_name(), d);
                let r = Rezzy::new(GREEN, msg);
                r.build_output();
                break
            } else if d < 1073741824000 && d > 137438953472 {
                let msg = format!("Minimum disk size requirement reached on {:?} Current:{} Preferred:{}", disk.get_name(), d, "1TB");
                let r = Rezzy::new(YELLOW, msg);
                r.build_output();
                break
            } else {
                let msg = format!("Minimum disk size requirement reached on {:?} Current:{} Preferred:{}",disk.get_name(), d, "1TB");
                let r = Rezzy::new(RED, msg);
                r.build_output();
            }
        }
    
        // To refresh all system information:
        sys.refresh_all();
    
        // We show the processes and some of their information:
        for (pid, process) in sys.get_processes() {
            if process.name().eq("chrome") {
                println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
                break
            }
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
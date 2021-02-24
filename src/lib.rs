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
        println!("Valid8r Run for ETH1 Client ({:?}) & ETH2 Client ({:?})\n", self.eth1, self.eth2);
        println!("System Requirements:");
        self.sys_req();

        println!("\nNetwork Requirements:");

        println!("\nETH1 Requirements:");

        println!("\nETH2 Requirements:");

        // self.net_req();
        // println!("done with net\n\n");
    
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
    
        let os = sys.get_name().unwrap().to_lowercase();
        // check os ver
        if os.eq("ubuntu") {
            let lts = "20.04";
            let cur = sys.get_os_version().unwrap();
            if cur.eq(lts) {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_green();
            } else {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_red();
            }
        } else if os.eq("darwin") {
            let lts = "11.2.1";
            let cur = sys.get_os_version().unwrap();
            if cur.eq("11.2.1") {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
                msg.write_green();
            } else {
                let msg = Rezzy{ message: format!("OS Version up-to-date with LTS: \n\t Requirement {:?} => Have ({:?} {:?})", lts, os, cur) };
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
            let msg = Rezzy{ message: format!("Memory requirement not reached: \n\t Preferred 16GB(min 8GB) => Have {} KB", mem) };
            msg.write_red();
        }
    
        // check num processors
        let proc = sys.get_processors().len();
        if proc > 4 {
            let msg = Rezzy{ message: format!("Processor count requirement reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_green();
        } else if proc < 4 && proc > 2 {
            let msg = Rezzy{ message: format!("Processor count requirement reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_yellow();
        } else {
            let msg = Rezzy{ message: format!("Processor count requirement reached: \n\t Preferred 4 CPU(s)(min 2) => Have {} CPU(s)", proc) };
            msg.write_red();
        }
    
        // // check disk size
        // for disk in sys.get_disks() {
        //     let d = disk.get_total_space();
        //     if d > 1000000000000 {
        //         let msg = format!("Disk size requirement reached: \n\t Preffered 1TB(min 128GB) => Have {:?}", disk.get_name(), d);
        //         let r = Rezzy::new(GREEN, msg);
        //         r.build_output();
        //         break
        //     } else if d < 1000000000000 && d > 128000000000 {
        //         let msg = format!("Minimum disk size requirement reached on {:?} Current:{} Preferred:{}", disk.get_name(), d, "1TB");
        //         let r = Rezzy::new(YELLOW, msg);
        //         r.build_output();
        //         break
        //     } else {
        //         let msg = format!("Minimum disk size requirement reached on {:?} Current:{} Preferred:{}",disk.get_name(), d, "1TB");
        //         let r = Rezzy::new(RED, msg);
        //         r.build_output();
        //     }
        // }
    
        // To refresh all system information:
        // sys.refresh_all();
    
        // // We show the processes and some of their information:
        // for (pid, process) in sys.get_processes() {
        //     if process.name().eq("chrome") {
        //         println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
        //         break
        //     }
        // }
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
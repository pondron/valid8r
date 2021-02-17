use sysinfo::{NetworksExt, System, SystemExt};
use std::net::TcpListener;
use std::io::{Error, ErrorKind};

pub fn net_req() {
    let s = System::new_all();
    for net in s.get_networks() {
        println!("{:?}", net);
    }

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

}
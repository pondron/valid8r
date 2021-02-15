use sysinfo::{ProcessExt, System, SystemExt, DiskExt};

pub fn sys_req() {
    let mut sys = System::new_all();

    // check os ver
    if sys.get_name().unwrap().eq("Ubuntu") {
        if sys.get_os_version().unwrap().eq("20.04") {
            println!("OS Ver OKAY: {:?}", sys.get_os_version());
        } else {
            println!("OS Ver NOT OKAY!!!!!")
        }
    }

    // check sys memory
    let mem = sys.get_total_memory();
    if mem > 17179869 {
        println!("All good have {}", mem);
    } else if mem < 17179869 && mem > 8589934 {
        println!("Min reached {} preferred {}", mem, 17179869);
    } else {
        println!("BAD BAD have {} need min {}", mem, 8589934);
    }

    // check num processors
    let proc = sys.get_processors().len();
    if proc > 4 {
        println!("All good have {}", proc);
    } else if proc < 4 && proc > 2 {
        println!("Min reached {} preferred {}", proc, 4);
    } else {
        println!("BAD BAD have {} need min {}", proc, 4);
    }

    // check disk size
    for disk in sys.get_disks() {
        let d = disk.get_total_space();
        if d > 1073741824000 {
            println!("All good {:?} have {}", disk.get_name(), d);
            break
        } else if d < 1073741824000 && d > 137438953472 {
            println!("Min reached {:?} {} preferred {}", disk.get_name(), d, "1TB");
            break
        } else {
            println!("BAD BAD have {:?} {} need min {}",disk.get_name(), d, "1TB");
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

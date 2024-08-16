//use bytesize::ByteSize;
//use colored::Colorize;
use compound_duration::format_dhms;
//use fs2::{available_space, total_space};
//use std::time::Instant;
use sysinfo::System; //Components, Disks, Networks, System};
use whoami;
use std::env;

fn main() {
    // let cyan ="\x1b[0;36m";
    // let bold = "\x1b[1m";
    let reset_col = "\x1b[0m";
    let cybold = "\x1b[1;36m";

    let name = System::name().unwrap();
    let ver = System::os_version().unwrap();
    let username = whoami::username();
    let hostname = System::host_name().unwrap();
    let fullnaming = format!("{}{}{}@{}{}{}", cybold, username, reset_col, cybold, hostname, reset_col);
    let namelen = format!("{}@{}", username, hostname).len();
    let possible_arch = System::cpu_arch().unwrap();
    let kernel = System::kernel_version().unwrap();
    let osname = env::consts::OS;

    println!("\n{}\n{}", fullnaming, "-".repeat(namelen));
    println!("{}OS{}: {} {} {:?}", cybold, reset_col, name, ver, possible_arch);
    println!("{}Host{}: {}", cybold, reset_col, hostname);
    println!("{}Kernel{}: {} {}", cybold, reset_col, osname, kernel);
    println!("{}Uptime{}: {}\n", cybold, reset_col, format_dhms(System::uptime()));

    //let se = "Boot Time".bold().blue();
    //println!("{}: {}",se, System::boot_time());

    //let se = "Memory".bold().blue();
    //println!("{}: {} / {}", se, ByteSize(sys.used_memory()), ByteSize(sys.total_memory()));

    /* let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let se = ("Disk").bold().blue();
        let edede = format!("({:?})", disk.mount_point()).bold().blue();
        println!("{} {}: {} / {}", se, edede, ByteSize(disk.total_space() - disk.available_space()),ByteSize(disk.total_space()),);
    } */
}

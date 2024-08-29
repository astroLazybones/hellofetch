//use bytesize::ByteSize;
//use colored::Colorize;
use compound_duration::format_dhms;
//use fs2::{available_space, total_space};
//use std::time::Instant;
use local_ip_address::list_afinet_netifas;
use local_ip_address::local_ip;
use resolution;
use std::env;
use sysinfo::System; //Components, Disks, Networks, System};
use whoami;

// original:
// a small grey cat named Skibidi drinks water from a small red bowl, while the neighbors dog Bowser laughs at Skibidi. Skibidi's owner, John, kills Bowser with his rizzling.

/* 100x translations:
The blue cat gave Scabby a drink from a small red cup and Bowser smiled at Scabby. Squidward's boss, John Bowser, lost the match.

The black cat drinks from a small red cup, John Bowser and his neighbors eat cookies.

A little girl named Scabby drinks from a red cup while her neighbor's dog Bowser laughs at Scabby. Kill John Bowser with the mirror.
*/

fn main() {
    // let cyan ="\x1b[0;36m";
    // let bold = "\x1b[1m";
    let reset_col = "\x1b[0m";
    let cybold = "\x1b[1;36m";
    let name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let ver = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let username = whoami::username();
    let screenres = resolution::current_resolution().unwrap();
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let fullnaming = format!(
        "{}{}{}@{}{}{}",
        cybold, username, reset_col, cybold, hostname, reset_col
    );
    let namelen = format!("{}@{}", username, hostname).len();
    let possible_arch = System::cpu_arch().unwrap_or_else(|| "Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let ipee = format!("{:?}", local_ip());
    let osname = env::consts::OS;
    let network_interfaces = list_afinet_netifas().unwrap();

    println!("\n{}\n{}", fullnaming, "-".repeat(namelen));
    println!(
        "{}OS{}: {} {} {:?}",
        cybold, reset_col, name, ver, possible_arch
    );
    println!("{}Host{}: {}", cybold, reset_col, hostname);
    println!("{}Kernel{}: {} {}", cybold, reset_col, osname, kernel);
    println!(
        "{}Uptime{}: {}",
        cybold,
        reset_col,
        format_dhms(System::uptime())
    );
    println!(
        "{}Display{}: {:?}x{:?}",
        cybold, reset_col, &screenres.0, &screenres.1,
    );

    for (name, ip) in network_interfaces.iter() {
        let d = format!("Ok({})", ip);
        if ipee == d {
            println!("{}Local IP ({}){}: {:?}", cybold, name, reset_col, ip);
        }
    }

    /*if let Ok(ip) = ip {
        println!("{}Local IP{}: {:?}", cybold, reset_col, ip);
    }*/

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
    println!("")
}

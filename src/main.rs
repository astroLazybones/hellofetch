use sysinfo::{
    Components, Disks, Networks, System,
};
use bytesize::ByteSize;
use colored::Colorize;
use fs2::{
    total_space, available_space,
};
use compound_duration::format_dhms;
use std::time::Instant;


fn main() {
    let name = System::name().unwrap();
    let ver = System::os_version().unwrap();
    let username = whoami::username();
    let hostname = System::host_name().unwrap();
    let fullnaming = format!("{}@{}", username.bold().blue(), hostname.bold().blue());
    let namelen = format!("{}@{}", username, hostname).len();
    println!("\n{}\n{}", fullnaming, "-".repeat(namelen));
    
    let se = "OS".bold().blue();
    let possible_arch = System::cpu_arch().unwrap();
    println!("{}: {} {} {:?}", se, name, ver, possible_arch);
    
    let se = "Host".bold().blue();
    let hostname = System::host_name().unwrap();
    println!("{}: {}",se, hostname);
    
    let se = "Kernel".bold().blue();
    let kernel = System::kernel_version().unwrap();
    println!("{}: {}", se, kernel);
    
    let se = "Uptime".bold().blue();
    println!("{}: {}",se, format_dhms(System::uptime()));
    
    //let se = "Boot Time".bold().blue();
    //println!("{}: {}",se, System::boot_time());
    
    //let se = "Memory".bold().blue();
    //println!("{}: {} / {}", se, ByteSize(sys.used_memory()), ByteSize(sys.total_memory()));
    
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {
        let se = ("Disk").bold().blue();
        let edede = format!("({:?})", disk.mount_point()).bold().blue();
        println!("{} {}: {} / {}", se, edede, ByteSize(disk.total_space() - disk.available_space()),ByteSize(disk.total_space()),);
    }
    println!("");
}

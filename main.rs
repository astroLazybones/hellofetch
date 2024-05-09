use sysinfo::{
    Components, Disks, Networks, System,
};
use bytesize::ByteSize;
use colored::Colorize;




fn main() {
    println!("");
    println!("hellofetch");
    println!("");
    
    let mut sys = System::new_all();
    
    let name = System::name().unwrap();
    let ver = System::os_version().unwrap();
    let se = "";
    
    
    let se = "OS:".bold().blue();
    println!("{}               {} {}", se, name, ver);
    
    let se = "Host:".bold().blue();
    let hostname = System::host_name().unwrap();
    println!("{}             {}",se, hostname);
    
    let se = "Kernel:".bold().blue();
    let kernel = System::kernel_version().unwrap();
    println!("{}           {}", se, kernel);
    
    let se = "Memory:".bold().blue();
    println!("{}           {} / {}", se, ByteSize(sys.used_memory()), ByteSize(sys.total_memory()));

    println!("");
}
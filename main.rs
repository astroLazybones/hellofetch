use rand::Rng; //0.8.5
use std::env;
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
    let mut se = "";    
    
    
    let mut se = "OS:".bold().blue();
    println!("{}               {} {}", se, name, ver);
    
    let mut se = "Host:".bold().blue();
    let hostname = System::host_name().unwrap();
    println!("{}             {}",se, hostname);
    
    let mut se = "Kernel:".bold().blue();
    let kernel = System::kernel_version().unwrap();
    println!("{}           {}", se, kernel);
    
    let mut se = "Memory:".bold().blue();
    println!("{}           {} / {}", se, ByteSize(sys.used_memory()), ByteSize(sys.total_memory()));

    println!("");
}

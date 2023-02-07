use sysinfo::{System,SystemExt,CpuExt};
use colored::*;
use std::env;
use std::fs::{self,File};
use std::io::Read;

struct Info {
    current_user_host: String,
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    shell: String,
    resolution: String,
    de: String,
    cpu: String,
    memory: String,
}

impl Info {
    pub fn new(sys: &System) -> Self {
        let current_user_host = Info::current_user(&sys);
        let os = sys.long_os_version().unwrap_or_else(||"<unkown>".to_owned());
        let host = Info::get_host();
        let kernel = Info::get_kernel_version(&sys);
        let uptime = Info::get_uptime(&sys);
        let shell = Info::get_shell_version();
        let resolution = Info::get_resolution();
        let de = Info::get_de();
        let cpu = Info::get_cpu(&sys);
        let memory = Info::get_memory(&sys);

        Self {
            current_user_host,
            os,
            host,
            kernel,
            uptime,
            shell,
            resolution,
            de,
            cpu,memory
        }
    }
    
    pub fn display(&self) {
        println!("
             {}
            {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            {}: {}
            ",
            self.current_user_host.blue(),
            "-------------------------------".red().bold(),
            "OS".blue().bold(), self.os,
            "Host".blue().bold(), self.host,
            "Kernel".blue().bold(), self.kernel,
            "Uptime".blue().bold(), self.uptime,
            "Shell".blue().bold(), self.shell,
            "Resoultion".blue().bold(), self.resolution,
            "DE".blue().bold(), self.de,
            "CPU".blue().bold(), self.cpu,
            "Memory".blue().bold(), self.memory
        );
    }

    pub fn get_de() -> String {
        env::var("XDG_CURRENT_DESKTOP").unwrap()
    }

    pub fn get_resolution() -> String {
        let contents = fs::read_to_string("/sys/class/graphics/fb0/virtual_size").unwrap();
        let dimensions: Vec<&str> = contents.trim().split(',').collect();
        let width = dimensions[0].parse::<u32>().unwrap();
        let height= dimensions[1].parse::<u32>().unwrap();
        format!("{width}x{height}")
    }

    pub fn get_shell_version() -> String {
        let shell_path = env::var("SHELL").unwrap();
        let parts: Vec<&str> = shell_path.split('/').collect();
        let shell_name = parts.last().unwrap();
        shell_name.to_string()
    }

    pub fn get_kernel_version(sys: &System) -> String {
        sys.kernel_version().unwrap_or_else(|| "<unknown>".to_owned())
    }

    pub fn get_host() -> String {
        let mut name = String::new();
        let mut version = String::new();
        match File::open("/sys/devices/virtual/dmi/id/product_name") {
            Ok(mut file) => {
                file.read_to_string(&mut name).unwrap();
            },
            Err(_) => {},
        };

        match File::open("/sys/devices/virtual/dmi/id/product_version") {
            Ok(mut file) => {
                file.read_to_string(&mut version).unwrap();
            },
            Err(_) => {},
        };

        format!("{} {}",name.trim(), version.trim() )
    }

    pub fn get_memory(sys: &System) -> String {
        let total = sys.total_memory() / (1024*1024);
        let used = sys.used_memory() / (1024*1024);
        format!("{used} MiB / {total} MiB")

    }

    pub fn get_uptime(sys: &System) -> String {
        let up = sys.uptime();
        let mut uptime = sys.uptime();
        let days = uptime / 86400;
        uptime -= days * 86400;
        let hours = uptime / 3600;
        uptime -= hours * 3600;
        let minutes = uptime / 60;
        format!("{days} days {hours} hours {minutes} minutes ({up} seconds)")
    }

    pub fn current_user(sys: &System) -> String {
        let current_user = env::var("USER").unwrap_or_else(|_| "<unkown>".to_owned()).blue();
        let host_name = sys.host_name().unwrap_or_else(|| "<unknown>".to_owned()).blue();
        format!("{current_user}@{host_name}")
    }

    pub fn get_cpu(sys: &System) -> String {
        let cpu = sys.global_cpu_info();
        cpu.brand().to_string()
        
    }

}


fn main() {
    let mut sys = System::new_all();
    sys.refresh_all(); // Update all information
    
    let info = Info::new(&sys);
    info.display();
    
}

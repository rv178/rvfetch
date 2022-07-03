use colored::Colorize;
use std::env;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let moon_art = format!(
        "     _.._          
  .' .-'`       
 /  /             
 |  |             
 \\  '.___.;       
  '._  _.'        
                "
    );
    let sys_name = format!("{}      - {}", "OS".blue().bold(), sys.name().unwrap().bold());
    let bar1 = "┌────────────────────────────────┐".bright_black();
    let bar2 = "└────────────────────────────────┘".bright_black();
    let sys_kern =
        format!("{}  - {}", "Kernel".blue().bold(), sys.kernel_version().unwrap().bold());
    let shell = format!("{}   - {}", "Shell".blue().bold(), env::var("SHELL").unwrap().bold());
    let mem_str = format!("{} MB / {} MB", sys.used_memory() / 1000, sys.total_memory() / 1000);
    let mem = format!("{}  - {}", "Memory".blue().bold(), mem_str.bold());
    let hostname =
        format!("{}@{}", env::var("USER").unwrap().green(), sys.host_name().unwrap().green());

    let moon_art = moon_art.split("\n").collect::<Vec<&str>>();
    let fields: Vec<String> =
        vec![hostname, bar1.to_string(), sys_name, sys_kern, shell, mem, bar2.to_string()];

    for i in 0..moon_art.len() {
        print!("{}", moon_art[i].yellow());
        print!("{}", fields[i]);

        print!("\n");
    }
}

use colored::Colorize;
use std::env;
use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // art
    let moon_art = format!(
        "     _.._          
  .' .-'`       
 /  /             
 |  |             
 \\  '.___.;       
  '._  _.'        
                "
    );

    // user@hostname
    let hostname: String =
        format!("{}@{}", env::var("USER").unwrap().green(), sys.host_name().unwrap().green());

    // start bar
    let start_bar: String = "┌──────────────────────────────────┐".bright_black().to_string();

    // distro name
    let sys_name: String = format!("{}      - {}", "OS".blue().bold(), sys.name().unwrap().bold());

    // kernel version
    let sys_kern: String =
        format!("{}  - {}", "Kernel".blue().bold(), sys.kernel_version().unwrap().bold());

    // shell name
    let shell: String =
        format!("{}   - {}", "Shell".blue().bold(), env::var("SHELL").unwrap().bold());

    // memory used (mb) / total memory (mb)
    let mem_str: String =
        format!("{} MB / {} MB", sys.used_memory() / 1000, sys.total_memory() / 1000);
    let mem: String = format!("{}  - {}", "Memory".blue().bold(), mem_str.bold());

    // end bar
    let end_bar: String = "└──────────────────────────────────┘".bright_black().to_string();

    // vec containing each line of moon_art
    let moon_art: Vec<&str> = moon_art.split("\n").collect::<Vec<&str>>();

    // all fields
    let fields: Vec<String> = vec![hostname, start_bar, sys_name, sys_kern, shell, mem, end_bar];

    for i in 0..moon_art.len() {
        print!("{}", moon_art[i].yellow());
        print!("{}", fields[i]);

        print!("\n");
    }
}

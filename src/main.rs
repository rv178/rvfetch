use std::{env, process::exit};
use sysinfo::{System, SystemExt};

fn main() {
    let moon_art = "     _.._\t
  .' .-'`\t
 /  /     \t
 |  |     \t
 \\  '.___.;  \t
  '._  _.' \t
\t\t";

    let arch_art = "       .\t
      / \\\t
     /   \\\t
    /^.   \\\t
   /  .-.  \\\t
  /  (   ) _\\\t
 /.^       ^.\\\t";

    let moon_art: Vec<&str> = moon_art.split('\n').collect::<Vec<&str>>();
    let arch_art: Vec<&str> = arch_art.split('\n').collect::<Vec<&str>>();

    let args: Vec<String> = env::args().collect();

    let gray = "\x1b[38;5;8m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let blue = "\x1b[34m";
    let reset = "\x1b[0m";

    let mut fields: Vec<String> = Vec::new();

    macro_rules! field {
        ($x:expr, $y:expr) => {
            fields.push(format!("  {}{}{} : {}", blue, $x, reset, $y))
        };
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let mem_str = format!("{} MB / {} MB", sys.used_memory() / 1000, sys.total_memory() / 1000);

    let hostname = format!(
        "{}{}{}@{}{}{}",
        green,
        env::var("USER").expect("Failed to get username."),
        reset,
        green,
        sys.host_name().expect("Failed to get hostname."),
        reset
    );

    field!(" ", hostname);
    fields.push(format!("{}┌──────────────────────────────────┐{}", gray, reset));
    field!(" ", sys.name().expect("Failed to get OS name."));
    field!(" ", sys.kernel_version().expect("Failed to get kernel info."));
    field!(" ", env::var("SHELL").expect("Failed to get shell info."));
    field!(" ", mem_str);
    fields.push(format!("{}└──────────────────────────────────┘{}", gray, reset));

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                help();
            }
            "-m" | "--moon" => {
                println!();
                for i in 0..moon_art.len() {
                    print!("{}{}{}", yellow, moon_art[i], reset);
                    print!("{}", fields[i]);
                    println!();
                }
            }
            _ => {
                println!("Invalid option '{}'.", args[1]);
                exit(1);
            }
        }
    } else if args.len() == 1 {
        println!();
        for i in 0..arch_art.len() {
            print!("{}{}{}", blue, arch_art[i], reset);
            print!("{}", fields[i]);
            println!();
        }
    }
}

fn help() {
    let help_msg = format!(
        "\x1b[32m\x1b[1mrvfetch \x1b[0m {}
    Simple fetch utility.

\x1b[33mUSAGE:\x1b[0m
    rvfetch \x1b[32m[OPTIONS]\x1b[0m

\x1b[33mOPTIONS:\x1b[0m
    \x1b[32m-h, --help\x1b[0m
        Show this help message.
    \x1b[32m-m, --moon\x1b[0m
        Show moon ASCII art.

Link: \x1b[4m\x1b[34mhttps://github.com/rv178/rvfetch\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", help_msg);
}

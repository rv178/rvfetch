use std::env;
use sysinfo::{System, SystemExt};

fn main() {
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
    let moon_art = "     _.._\t
  .' .-'`\t
 /  /     \t
 |  |     \t
 \\  '.___.;  \t
  '._  _.' \t
\t\t";

    let moon_art: Vec<&str> = moon_art.split('\n').collect::<Vec<&str>>();

    let hostname = format!(
        "{}{}{}@{}{}{}",
        green,
        env::var("USER").unwrap(),
        reset,
        green,
        sys.host_name().unwrap(),
        reset
    );

    field!(" ", hostname);
    fields.push(format!("{}┌──────────────────────────────────┐{}", gray, reset));
    field!(" ", sys.name().unwrap());
    field!(" ", sys.kernel_version().unwrap());
    field!(" ", env::var("SHELL").unwrap());
    field!(" ", mem_str);
    fields.push(format!("{}└──────────────────────────────────┘{}", gray, reset));

    println!();
    for i in 0..moon_art.len() {
        print!("{}{}{}", yellow, moon_art[i], reset);
        print!("{}", fields[i]);
        println!();
    }
}

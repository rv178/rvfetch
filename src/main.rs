use std::env;
use sysinfo::{System, SystemExt};

fn main() {
    let gra = "\x1b[38;5;8m";
    let gre = "\x1b[32m";
    let yel = "\x1b[33m";
    let blu = "\x1b[34m";
    let bld = "\x1b[1m";
    let res = "\x1b[0m";

    let mut fields: Vec<String> = Vec::new();

    macro_rules! field {
        ($x:expr, $y:expr) => {
            fields.push(format!("{}{}{} - {}{}{}", blu, $x, res, bld, $y, res))
        };
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let mem_str = format!("{} MB / {} MB", sys.used_memory() / 1000, sys.total_memory() / 1000);
    let moon_art = format!(
        "     _.._          
  .' .-'`       
 /  /             
 |  |             
 \\  '.___.;       
  '._  _.'        
                "
    );
    let moon_art: Vec<&str> = moon_art.split("\n").collect::<Vec<&str>>();

    fields.push(format!(
        "{}{}@{}{}",
        gre,
        env::var("USER").unwrap(),
        sys.host_name().unwrap(),
        res
    ));

    fields.push(format!("{}┌──────────────────────────────────┐{}", gra, res));
    field!("OS", sys.name().unwrap());
    field!("KR", sys.kernel_version().unwrap());
    field!("SH", env::var("SHELL").unwrap());
    field!("ME", mem_str);
    fields.push(format!("{}└──────────────────────────────────┘{}", gra, res));

    for i in 0..moon_art.len() {
        print!("{}{}{}", yel, moon_art[i], res);
        print!("{}", fields[i]);

        print!("\n");
    }
}

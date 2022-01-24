use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_distro() -> String {
    let mut os_release = String::new();
    if let Ok(lines) = read_lines("/etc/os-release") {
        for line in lines {
            if let Ok(value) = line {
                if value.starts_with("PRETTY_NAME=\"") {
                    let len = value.len();
                    os_release = (&String::from(value)[13..len - 1]).to_string();
                }
            }
        }
    }
    os_release
}

fn get_kernel() -> String {
    let data = fs::read_to_string("/proc/version").expect("Unable to read file");
    let i1 = data.find("version ").unwrap() + 8;
    let i2 = data.find(" (").unwrap();
    // println!("Hello, {} {}, finally: {}", i1, i2, &data[i1..i2]);
    (&data[i1..i2]).to_string()
}

fn get_uptime() -> String {
    let data = fs::read_to_string("/proc/uptime").unwrap();
    let numbers: Vec<&str> = data.split(' ').collect();
    let uptime: Vec<&str> = numbers[0].split('.').collect();
    let mut uptime_string: String = String::new();
    let upt: i32 = uptime[0].parse().unwrap();
    let d = upt / (3600 * 24);
    let h = upt % (3600 * 24) / 3600;
    let m = upt % 3600 / 60;

    if d > 0 {
        uptime_string.push_str(&format!("{}d ", h));
    }
    if h > 0 {
        uptime_string.push_str(&format!("{}h ", h));
    }
    if m > 0 {
        uptime_string.push_str(&format!("{}m", m));
    }

    uptime_string
}

fn get_shell() -> String {
    let mut shell: String = String::new();

    match env::var("SHELL") {
        Ok(val) => {
            let last: Vec<&str> = val.split('/').collect();
            shell = last.last().cloned().unwrap().to_string();
        }
        Err(e) => println!("Error happened with: {}", e),
    }
    shell
}

fn get_wm() -> String {
    let mut wm: String = String::new();

    match env::var("DESKTOP_SESSION") {
        Ok(val) => {
            wm = val.to_string();
        }
        Err(e) => println!("Error happened with: {}", e),
    }
    wm
}

fn get_term() -> String {
    let mut term: String = String::new();

    match env::var("TERM") {
        Ok(val) => {
            if val.contains("xterm-") {
                let last: Vec<&str> = val.split('-').collect();
                term = last.last().cloned().unwrap().to_string();
            } else {
                term = val.to_string();
            }
        }
        Err(e) => println!("Error happened with: {}", e),
    }
    term
}

fn main() {
    println!("Hello world!!");
    // for (key, value) in env::vars_os() {
    //     println!("{:?}: {:?}", key, value);
    // }
    let distro = get_distro();
    let kernel = get_kernel();
    let uptime = get_uptime();
    let shell = get_shell();
    let wm = get_wm();
    let term = get_term();

    println!("OS: {}", distro);
    println!("Kernel: {}", kernel);
    println!("Uptime: {}", uptime);
    println!("Shell: {}", shell);
    println!("WM: {}", wm);
    println!("Terminal: {}", term);
}

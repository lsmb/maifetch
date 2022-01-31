use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Data {
    pub name: String,
    pub icon: char,
    pub value: String,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_username() -> String {
    let mut username: String = String::new();

    match env::var("USER") {
        Ok(val) => {
            username = val.to_string();
        }
        Err(e) => println!("Error happened with: {}", e),
    }
    username
}

fn get_hostname() -> String {
    let mut hostname = String::new();
    if let Ok(lines) = read_lines("/proc/sys/kernel/hostname") {
        for line in lines {
            if let Ok(value) = line {
                hostname = value.to_string();
            }
        }
    }
    hostname
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

pub fn username() -> Data {
    Data {
        name: "User".to_string(),
        icon: '\u{f17c}',
        value: get_username(),
    }
}

pub fn hostname() -> Data {
    Data {
        name: "Hostname".to_string(),
        icon: '\u{f17c}',
        value: get_hostname(),
    }
}

pub fn distro() -> Data {
    Data {
        name: "OS".to_string(),
        icon: '\u{f17c}',
        value: get_distro(),
    }
}

pub fn kernel() -> Data {
    Data {
        name: "Kernel".to_string(),
        icon: '\u{e266}',
        value: get_kernel(),
    }
}

pub fn uptime() -> Data {
    Data {
        name: "Uptime".to_string(),
        icon: '\u{f017}',
        value: get_uptime(),
    }
}

pub fn shell() -> Data {
    Data {
        name: "Shell".to_string(),
        icon: '\u{e795}',
        value: get_shell(),
    }
}

pub fn wm() -> Data {
    Data {
        name: "WM".to_string(),
        icon: '\u{f878}',
        value: get_wm(),
    }
}

pub fn term() -> Data {
    Data {
        name: "Term".to_string(),
        icon: '\u{f44f}',
        value: get_term(),
    }
}

use clap::{Parser, Subcommand};
use image;
use image::GenericImageView;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use termion::{clear, color, cursor, style};
use viuer::Config;

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

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Help {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,

    /// Sets a custom image file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    image: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    /// Set text / text
    #[clap(short, long)]
    text: bool,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long)]
        list: bool,
    },
}

fn render_image(path: &Path) -> (f32, f32) {
    let term_size_pixels = termion::terminal_size_pixels().unwrap();
    let x_size = term_size_pixels.0 as u32 / 2;
    let y_size = term_size_pixels.1 as f32 / 1.2;

    let img = image::open(path).unwrap();
    let img_resized = img.resize(x_size, y_size as u32, image::imageops::Lanczos3);

    print!("{}", clear::All);

    let conf = Config {
        // set offset
        x: 0,
        y: 0,
        // set dimensions
        width: Some(0),
        height: Some(0),
        ..Default::default()
    };

    viuer::print(&img_resized, &conf).expect("Image printing failed.");
    let resized_sizes = img_resized.dimensions();

    (
        term_size_pixels.0 as f32 / resized_sizes.0 as f32,
        term_size_pixels.1 as f32 / resized_sizes.1 as f32,
    )
}

fn render_ascii() -> (f32, f32) {
    println!("{}{}", clear::All, cursor::Goto(1, 1));
    let zerotwo_art: &[&str] = &[
        "⣿⣿⣿⣿⣯⣿⣿⠄⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠈⣿⣿⣿⣿⣿⣿⣆⠄",
        "⢻⣿⣿⣿⣾⣿⢿⣢⣞⣿⣿⣿⣿⣷⣶⣿⣯⣟⣿⢿⡇⢃⢻⣿⣿⣿⣿⣿⢿⡄",
        "⠄⢿⣿⣯⣏⣿⣿⣿⡟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣧⣾⢿⣮⣿⣿⣿⣿⣾⣷",
        "⠄⣈⣽⢾⣿⣿⣿⣟⣄⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣝⣯⢿⣿⣿⣿⣿",
        "⣿⠟⣫⢸⣿⢿⣿⣾⣿⢿⣿⣿⢻⣿⣿⣿⢿⣿⣿⣿⢸⣿⣼⣿⣿⣿⣿⣿⣿⣿",
        "⡟⢸⣟⢸⣿⠸⣷⣝⢻⠘⣿⣿⢸⢿⣿⣿⠄⣿⣿⣿⡆⢿⣿⣼⣿⣿⣿⣿⢹⣿",
        "⡇⣿⡿⣿⣿⢟⠛⠛⠿⡢⢻⣿⣾⣞⣿⡏⠖⢸⣿⢣⣷⡸⣇⣿⣿⣿⢼⡿⣿⣿",
        "⣡⢿⡷⣿⣿⣾⣿⣷⣶⣮⣄⣿⣏⣸⣻⣃⠭⠄⠛⠙⠛⠳⠋⣿⣿⣇⠙⣿⢸⣿",
        "⠫⣿⣧⣿⣿⣿⣿⣿⣿⣿⣿⣿⠻⣿⣾⣿⣿⣿⣿⣿⣿⣿⣷⣿⣿⣹⢷⣿⡼⠋",
        "⠄⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⣿⣿⣿⠄⠄",
        "⠄⠄⢻⢹⣿⠸⣿⣿⣿⣿⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣼⣿⣿⣿⣿⡟⠄⠄",
        "⠄⠄⠈⢸⣿⠄⠙⢿⣿⣿⣹⣿⣿⣿⣿⣟⡃⣽⣿⣿⡟⠁⣿⣿⢻⣿⣿⢿⠄⠄",
        "⠄⠄⠄⠘⣿⡄⠄⠄⠙⢿⣿⣿⣾⣿⣷⣿⣿⣿⠟⠁⠄⠄⣿⣿⣾⣿⡟⣿⠄⠄",
        "⠄⠄⠄⠄⢻⡇⠸⣆⠄⠄⠈⠻⣿⡿⠿⠛⠉⠄⠄⠄⠄⢸⣿⣇⣿⣿⢿⣿⠄⠄",
    ];

    for line in zerotwo_art.iter() {
        println!("{}", line);
    }

    let term_size = termion::terminal_size().unwrap();

    (
        term_size.0 as f32 / zerotwo_art[0].chars().count() as f32,
        term_size.1 as f32 / zerotwo_art.len() as f32,
    )
}

fn render(
    user: (String, String),
    infos: Vec<((&str, &str), String)>,
    ratio: (f32, f32),
    text: bool,
) {
    // let term_size_pixels = termion::terminal_size_pixels().unwrap();
    let term_size = termion::terminal_size().unwrap();
    let mut counter = (term_size.1 as f32 / ratio.1 as f32 / 2.3) as u16;

    let offset_x = (term_size.0 as f32 / ratio.0) as u16 + 4;
    let offset_y = (term_size.1 as f32 / ratio.1) as u16 + 1;

    print!(
        "{}{}{}{}{}@{}{}",
        cursor::Goto(offset_x, counter),
        style::Bold,
        color::Fg(color::Red),
        user.0,
        color::Fg(color::Cyan),
        color::Fg(color::Red),
        user.1
    );
    print!(
        "{}{}{}",
        cursor::Goto(offset_x, counter + 1),
        color::Fg(color::Magenta),
        "—".repeat(user.0.len() + user.1.len() + 1)
    );
    counter = counter + 2;

    if !text {
        for info in infos {
            print!(
                "{}{}{}{} ->{} {}",
                cursor::Goto(offset_x, counter),
                style::Bold,
                color::Fg(color::Magenta),
                info.0 .0,
                color::Fg(color::White),
                info.1
            );
            counter = counter + 1;
        }
    } else {
        let mut string_lengths: Vec<usize> = Vec::new();
        for v in infos.clone() {
            string_lengths.push(v.0 .1.chars().count())
        }

        let padding = string_lengths.iter().max().unwrap();
        for info in infos {
            let pad = String::from_utf8(vec![b' '; padding - info.0 .1.chars().count()]).unwrap();
            print!(
                "{}{}{}{}{} ->{} {}",
                cursor::Goto(offset_x, counter),
                style::Bold,
                color::Fg(color::Magenta),
                info.0 .1,
                pad,
                color::Fg(color::White),
                info.1
            );
            counter = counter + 1;
        }
    }

    print!(
        "{}{}███{}███{}███{}███{}███{}███{}███{}███",
        cursor::Goto(offset_x, counter + 1),
        color::Fg(color::Red),
        color::Fg(color::Yellow),
        color::Fg(color::Green),
        color::Fg(color::Cyan),
        color::Fg(color::Blue),
        color::Fg(color::Magenta),
        color::Fg(color::Black),
        color::Fg(color::White),
    );
    print!("{}", cursor::Goto(1, offset_y + 1));
}

fn main() {
    let help = Help::parse();

    match &help.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // NOTE: To debug environment variables
    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }

    let username = get_username();
    let hostname = get_hostname();
    let distro = get_distro();
    let kernel = get_kernel();
    let uptime = get_uptime();
    let shell = get_shell();
    let wm = get_wm();
    let term = get_term();

    let mut infos: Vec<((&str, &str), String)> = Vec::new();
    let user = (username, hostname);

    infos.push((("\u{f17c}", "OS"), distro));
    infos.push((("\u{e266}", "Kernel"), kernel));
    infos.push((("\u{f017}", "Uptime"), uptime));
    infos.push((("\u{e795}", "Shell"), shell));
    infos.push((("\u{f878}", "WM"), wm));
    infos.push((("\u{f44f}", "Term"), term));

    let mut text: bool = false;

    if help.text {
        text = help.text;
    }

    if let Some(name) = help.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = help.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    let ratio;
    // If -i/--image argument given, render image
    if let Some(image) = help.image.as_deref() {
        ratio = render_image(image);
    } else {
        ratio = render_ascii();
    }

    render(user, infos, ratio, text);
}

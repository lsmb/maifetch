use image as imageutil;
use image::GenericImageView;
use std::path::{Path, PathBuf};
use termion::{clear, color, cursor, style};

use crate::data;
use viuer::Config;

#[derive(Clone)]
pub struct RenderConfig {
    pub data: Vec<data::Data>,
    pub user: (data::Data, data::Data),
    pub icons: bool,
    pub ratio: (f32, f32),
}

impl RenderConfig {
    fn set_ratio(&mut self, ratio: &(f32, f32)) {
        self.ratio = *ratio;
    }
}

impl Default for RenderConfig {
    fn default() -> Self {
        RenderConfig {
            data: vec![
                data::distro(),
                data::kernel(),
                data::uptime(),
                data::shell(),
                data::wm(),
                data::term(),
            ],
            user: (data::username(), data::hostname()),
            icons: true,
            ratio: (1.0, 1.0),
        }
    }
}

pub fn image(path: &Path, config: &mut RenderConfig) {
    let term_size_pixels = termion::terminal_size_pixels().unwrap();
    let x_size = term_size_pixels.0 as u32 / 2;
    let y_size = term_size_pixels.1 as f32 / 1.2;

    let img = imageutil::open(path).unwrap();
    let img_resized = img.resize(x_size, y_size as u32, imageutil::imageops::Lanczos3);

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

    let ratio = (
        term_size_pixels.0 as f32 / resized_sizes.0 as f32,
        term_size_pixels.1 as f32 / resized_sizes.1 as f32,
    );

    config.set_ratio(&ratio)
}

pub fn ascii(config: &mut RenderConfig) {
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

    let ratio = (
        term_size.0 as f32 / zerotwo_art[0].chars().count() as f32,
        term_size.1 as f32 / zerotwo_art.len() as f32,
    );

    config.set_ratio(&ratio)
}

pub fn data(config: RenderConfig) {
    // let term_size_pixels = termion::terminal_size_pixels().unwrap();
    let term_size = termion::terminal_size().unwrap();
    let mut counter = (term_size.1 as f32 / config.ratio.1 as f32 / 2.3) as u16;

    let offset_x = (term_size.0 as f32 / config.ratio.0) as u16 + 4;
    let offset_y = (term_size.1 as f32 / config.ratio.1) as u16 + 1;

    print!(
        "{}{}{}{}{}@{}{}",
        cursor::Goto(offset_x, counter),
        style::Bold,
        color::Fg(color::Red),
        config.user.0.value,
        color::Fg(color::Cyan),
        color::Fg(color::Red),
        config.user.1.value
    );
    print!(
        "{}{}{}",
        cursor::Goto(offset_x, counter + 1),
        color::Fg(color::Magenta),
        "—".repeat(config.user.0.value.len() + config.user.1.value.len() + 1)
    );
    counter = counter + 2;

    if config.icons {
        for data in config.data {
            print!(
                "{}{}{}{} ->{} {}",
                cursor::Goto(offset_x, counter),
                style::Bold,
                color::Fg(color::Magenta),
                data.icon,
                color::Fg(color::White),
                data.value
            );
            counter = counter + 1;
        }
    } else {
        let mut string_lengths: Vec<usize> = Vec::new();
        for v in config.data.clone() {
            string_lengths.push(v.name.chars().count())
        }

        let padding = string_lengths.iter().max().unwrap();
        for data in config.data {
            let pad = String::from_utf8(vec![b' '; padding - data.name.chars().count()]).unwrap();
            print!(
                "{}{}{}{}{} ->{} {}",
                cursor::Goto(offset_x, counter),
                style::Bold,
                color::Fg(color::Magenta),
                data.name,
                pad,
                color::Fg(color::White),
                data.value
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

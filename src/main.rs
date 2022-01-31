use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod data;
mod render;
use data::Data;
use render::RenderConfig;

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

    // // NOTE: To debug environment variables
    // for (key, value) in env::vars_os() {
    //     println!("{:?}: {:?}", key, value);
    // }
    let mut config = RenderConfig {
        ..Default::default()
    };

    if help.text {
        config.icons = false;
    }

    if let Some(name) = help.name.as_deref() {
        println!("Value for name: {}", name);
    }

    if let Some(config_path) = help.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    println!("{}", config.data[0].value);

    if let Some(image) = help.image.as_deref() {
        render::image(image, &mut config)
    } else {
        render::ascii(&mut config)
    }

    render::data(config);
}

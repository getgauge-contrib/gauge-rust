extern crate gauge_rs;

use std::env;

fn help() {
    println!("Rust language runner for Gauge. Version: {}",
             gauge_rs::version());
    println!("\nAvailable commands:");
    println!(" --init    - Create a new Gauge project with Rust language runner in the current \
              directory");
    println!(" --run     - Run the plugin. Needs to be invoked from Gauge");
    println!(" --version - Show plugin version");
    println!(" --help    - Show this help message");
}

fn print_version() {
    println!("{}", gauge_rs::version().trim_matches('"'));
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_ref() {
            "--run" => gauge_rs::runner::run(),
            "--init" => gauge_rs::install::init(),
            "--version" => print_version(),
            _ => help(),
        }
    } else {
        help()
    }
}

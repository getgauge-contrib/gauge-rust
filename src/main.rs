extern crate gauge_rs;

use std::env;

fn help () {
    println!("Help");
}

fn main () {
    if let Some(arg) = env::args().nth(1) {
        match arg.as_ref() {
            "--run" => gauge_rs::runner::run(),
            "--init" => gauge_rs::install::init(),
            _ => help()
        }
    } else {
        help()
    }
}

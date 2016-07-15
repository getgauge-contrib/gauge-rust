extern crate lazy_static;

use std::env;
use std::io::Result;
use std::path::PathBuf;
use std::fs::{self, DirEntry};
use std::string::String;

pub struct Config {
    pub steps_file: String,
    pub step_implementations_path: PathBuf,
    pub skel_path: PathBuf,
    pub internal_port: String,
}

lazy_static! {
    pub static ref PROJECT_ROOT: PathBuf = PathBuf::from(env_var("GAUGE_PROJECT_ROOT"));
    pub static ref PLUGIN_SOURCE: PathBuf = match env::current_dir() {
        Ok(d) => d,
        Err(_) => PathBuf::from("")
    };

    pub static ref CONFIG: Config = Config {
        steps_file: String::from("steps.rs"),
        step_implementations_path: path_to("tests", &PROJECT_ROOT),
        skel_path: path_to("skel", &PLUGIN_SOURCE),
        internal_port: env_var("GAUGE_INTERNAL_PORT"),
    };
}

pub fn env_var(ev: &'static str) -> String {
    match env::var(ev) {
        Ok(val) => val,
        Err(_) => String::new(),
    }
}

pub fn path_to<'a>(pathslice: &'a str, root: &PathBuf) -> PathBuf { root.join(pathslice) }

pub fn create_dir(dirpath: &PathBuf) -> Result<&PathBuf> {
    try!(fs::create_dir_all(dirpath));
    Ok(dirpath)
}

pub fn copy_file<'a>(from: &'a PathBuf, to: &'a PathBuf) -> Result<(&'a PathBuf, &'a PathBuf)> {
    try!(fs::copy(from, to));
    Ok((from, to))
}

pub fn visit_dirs(dir: &PathBuf, callback: &Fn(&DirEntry)) -> Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), callback));
            } else {
                callback(&entry);
            }
        }
    }
    Ok(())
}

use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};
use std::string::String;

pub enum Root {
    Plugin,
    Project,
}

pub struct Config {
    pub steps_file: &'static str,
    pub step_implementations_path: &'static str,
    pub skel_path: &'static str,
    pub internal_port: &'static str,
    pub project_root: &'static str,
}

pub const CONFIG: Config = Config {
    steps_file: "steps.rs",
    step_implementations_path: "tests",
    skel_path: "skel",
    internal_port: "GAUGE_INTERNAL_PORT",
    project_root: "GAUGE_PROJECT_ROOT",
};

pub fn env_var(ev: &'static str) -> Option<String> { env::var(ev).ok() }

pub fn path_to<'a>(path: &'a str, root: Root) -> PathBuf {
    let root_path = match root {
        Root::Plugin => env::current_dir().unwrap(),
        Root::Project => PathBuf::from(env_var(CONFIG.project_root).unwrap()),
    };
    root_path.join(path)
}

pub fn create_dir(dirpath: &PathBuf) {
    fs::create_dir_all(dirpath).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn copy_file(from: &PathBuf, to: &PathBuf) {
    match fs::copy(from, to) {
        Ok(_) => println!("Created: {:?}", to.file_name().unwrap()),
        Err(e) => {
            panic!("Could not create {:?}. Error: {}",
                   to.file_name().unwrap(),
                   e)
        }
    }
}

pub fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

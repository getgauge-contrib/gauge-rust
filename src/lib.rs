#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub mod common;
pub mod install;
pub mod runner;

pub fn plugin_json() -> Json {
    let file_path = Path::new("rust.json");
    let mut file = match File::open(&file_path) {
        Err(why) => panic!("Couldn't open plugin meta file {}: {}", file_path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read plugin meta file {}: {}", file_path.display(), Error::description(&why)),
        Ok(content) => content,
    };

    match Json::from_str(&s) {
        Err(why) => panic!("Couldn't parse plugin JSON: {}", Error::description(&why)),
        Ok(jsoncontent) => jsoncontent,
    }
}

pub fn version() -> String {
    let json = plugin_json();
    let ver = json.find_path(&["version"]).unwrap();
    ver.to_string()
}

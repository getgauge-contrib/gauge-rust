use common::*;
use std::fs::DirEntry;
use std::ffi::OsStr;

fn parse_test_file(entry: &DirEntry) {
    println!("Parsing: {:?}", entry.file_name());
    // TODO: Parse file here
}

fn scan_result(entry: &DirEntry) {
    let ext = OsStr::new("rs");
    if entry.path().extension() == Some(ext) {
        parse_test_file(entry);
    }
}

pub fn scan() {
    println!("Scanning: {:?}", &CONFIG.step_implementations_path);
    if let Err(e) =  visit_dirs(&CONFIG.step_implementations_path, &scan_result) {
        println!("Error trying to scan implementations: {:?}", e)
    }
}

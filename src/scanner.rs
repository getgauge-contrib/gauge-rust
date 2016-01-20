use common::*;
use std::fs::DirEntry;
use std::ffi::OsStr;

fn parse_test_file(entry: &DirEntry) {
    println!("Will parse: {:?}", entry.file_name());
}

fn scan_result(entry: &DirEntry) {
    let ext = OsStr::new("rs");
    if entry.path().extension() == Some(ext) {
        parse_test_file(entry);
    }
}

pub fn scan() { visit_dirs(&CONFIG.step_implementations_path, &scan_result).unwrap(); }

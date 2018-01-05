#![allow(warnings)]
// use std::fs;
// use std::path::PathBuf;

const proc_stat: &str = "/proc/stat";

pub fn is_available() -> bool {
    true // we always have a cpu
}

pub fn load() -> Vec<f64> {
    // let data = fs::read_file(proc_stat);

    Vec::new()
}

// read /proc/stat
// pub fn procstat() -> String {
// }

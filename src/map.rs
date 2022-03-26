use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Map {
    rows: Vec<Vec<i32>>,
}

impl Map {
    pub fn from_json_file(filename: &str) -> Self {
        serde_json::from_reader(BufReader::new(File::open(Path::new(filename)).unwrap())).unwrap()
    }
}

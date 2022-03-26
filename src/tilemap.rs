use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TileMapConfig {
    image_path: String,
    tiles: Vec<TileMetadata>,
}

impl TileMapConfig {
    pub fn from_json_file(filename: &str) -> Self {
        serde_json::from_reader(BufReader::new(File::open(Path::new(filename)).unwrap())).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct TileMetadata {
    top: i32,
    left: i32,
    name: i32,
    width: i32,
    height: i32,
}

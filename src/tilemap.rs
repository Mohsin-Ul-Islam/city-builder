use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TileMapConfig {
    // image_path: String,
    tiles: Vec<TileMetadata>,
}

impl TileMapConfig {
    pub fn from_json_file(filename: &str) -> Self {
        serde_json::from_reader(BufReader::new(File::open(Path::new(filename)).unwrap())).unwrap()
    }

    pub fn at(&self, tile_id: &i32) -> &TileMetadata {
        self.tiles.iter().find(|&tile| tile.id == *tile_id).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct TileMetadata {
    pub id: i32,
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
}

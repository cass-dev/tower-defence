use ron::de::from_reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Tiles {
    pub indexes: Vec<usize>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub tiles: Tiles,
}

impl Level {
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        let f = File::open(path).expect("failed to open file");
        match from_reader(f) {
            Ok(level) => level,
            Err(e) => panic!("failed to load level {}", e),
        }
    }
}

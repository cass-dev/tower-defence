use ron::de::from_reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path as StdPath;

#[derive(Debug, Deserialize)]
pub struct Tiles {
    pub indexes: Vec<usize>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize)]
pub enum TowerType {
    BASIC,
}

#[derive(Debug, Deserialize)]
pub struct Tower {
    pub tower_type: TowerType,
    pub position: (f32, f32),
}

#[derive(Debug, Deserialize)]
pub enum EnemyType {
    MAN,
}

#[derive(Debug, Deserialize)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub start_position: (f32, f32),
}

#[derive(Debug, Deserialize)]
pub struct Path {
    pub points: Vec<(f32, f32)>,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    pub tiles: Tiles,
    pub towers: Vec<Tower>,
    pub enemies: Vec<Enemy>,
    pub path: Path,
}

impl Level {
    pub fn from_file(path: impl AsRef<StdPath>) -> Self {
        let f = File::open(path).expect("failed to open file");
        match from_reader(f) {
            Ok(level) => level,
            Err(e) => panic!("failed to load level {}", e),
        }
    }
}

use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::{World, WorldExt};
use amethyst::{
    assets::Handle,
    assets::ProgressCounter,
    assets::{Asset, AssetStorage, Loader, ProcessableAsset, ProcessingState},
    assets::{Format, RonFormat},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    core::{
        bundle::SystemBundle,
        ecs::{prelude::*, Entity},
    },
    Result,
};
use ron::de::from_reader;
use ron::de::Deserializer;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::fs::File;
use std::path::Path as StdPath;

#[derive(Debug, Deserialize, Clone)]
pub struct Tiles {
    pub indexes: Vec<usize>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TowerType {
    BASIC,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tower {
    pub tower_type: TowerType,
    pub position: (f32, f32),
}

#[derive(Debug, Deserialize, Clone)]
pub enum EnemyType {
    MAN,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub start_position: (f32, f32),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Path {
    pub points: Vec<(f32, f32)>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Level {
    pub tiles: Tiles,
    pub towers: Vec<Tower>,
    pub enemies: Vec<Enemy>,
    pub path: Path,
}

impl Asset for Level {
    const NAME: &'static str = "tower-defence::Level";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<Self>>;
}

pub fn init(world: &mut World, progress: &mut RefCell<ProgressCounter>) {
    let handle: Handle<Level> = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "levels/level_1.ron",
            RonFormat,
            progress.get_mut(),
            &world.read_resource(),
        )
    };
    world.insert(handle);
}

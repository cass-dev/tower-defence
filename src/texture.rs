use amethyst::{
    assets::ProgressCounter,
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};
use std::cell::RefCell;

pub struct SpriteSheetHandle(pub Handle<SpriteSheet>);

pub fn init(world: &mut World, progress_counter: &mut RefCell<ProgressCounter>) {
    let sprite_sheet_handle = get_sprite_sheet_handle(
        world,
        "textures/sprite_sheet.png",
        "textures/sprite_sheet.ron",
        progress_counter,
    );

    world.insert(SpriteSheetHandle(sprite_sheet_handle));
}

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    mut progress_counter: &mut RefCell<ProgressCounter>,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_path,
            ImageFormat::default(),
            progress_counter.get_mut(),
            &texture_storage,
        )
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter.get_mut(),
        &sprite_sheet_store,
    )
}

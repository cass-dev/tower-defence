use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    assets::ProgressCounter,
};

pub struct SpriteSheetHandle(pub Handle<SpriteSheet>);

pub fn init(world: &mut World) -> ProgressCounter {
    let mut progress_counter = Some(ProgressCounter::new());

    let sprite_sheet_handle = get_sprite_sheet_handle(world, "textures/sprite_sheet.png", "textures/sprite_sheet.ron", &mut progress_counter);

    world.insert(SpriteSheetHandle(sprite_sheet_handle));

    progress_counter.unwrap()
}

pub fn get_sprite_sheet_handle(
    world: &World,
    texture_path: &str,
    ron_path: &str,
    mut progress_counter: &mut Option<ProgressCounter>,
) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = &world.read_resource::<Loader>();
        let texture_storage = &world.read_resource::<AssetStorage<Texture>>();
        loader.load(texture_path, ImageFormat::default(), progress_counter.as_mut().unwrap(), &texture_storage)
    };
    let loader = &world.read_resource::<Loader>();
    let sprite_sheet_store = &world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress_counter.as_mut().unwrap(),
        &sprite_sheet_store,
    )
}

use crate::level::Level;
use amethyst::core::transform::TransformBundle;
use amethyst::{
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod camera;
mod components;
mod constants;
mod level;
mod resources;
mod states;
mod systems;
mod texture;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.008, 0.043, 0.067, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?;

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, states::Load::default(), game_data)?;
    game.run();

    Ok(())
}

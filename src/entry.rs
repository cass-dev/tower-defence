use crate::level::Level;
use crate::states;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::rendy::hal::command::ClearColor;
use amethyst::window::{DisplayConfig, EventLoop};
use amethyst::{
    assets::Processor,
    input::{Bindings, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
use std::path::Path;

pub fn run_application<FnSetup>(setup_fn: FnSetup) -> amethyst::Result<()>
where
    FnSetup:
        FnOnce(
            &Path,
            &EventLoop<()>,
        )
            -> amethyst::Result<(Bindings<StringBindings>, RenderingBundle<DefaultBackend>)>,
{
    let app_root = application_root_dir()?;
    let event_loop = EventLoop::new();

    let (bindings, rendering_bundle) = setup_fn(&app_root, &event_loop)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            rendering_bundle
                .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                    float32: [0.1, 0.1, 0.1, 1.0],
                }))
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings(bindings))?
        .with(Processor::<Level>::new(), "settings_processor", &[]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, states::Load::default(), game_data)?;

    game.run_winit_loop(event_loop);

    Ok(())
}

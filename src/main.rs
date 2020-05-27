use crate::level::Level;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::rendy::hal::command::ClearColor;
use amethyst::window::{DisplayConfig, EventLoop};
use amethyst::{
    input::{Bindings, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
};
use std::path::Path;

mod camera;
mod components;
mod constants;
mod entry;
mod level;
mod physics;
mod resources;
mod states;
mod systems;
mod texture;
mod wasm;

#[allow(unused)]
#[cfg(not(feature = "wasm"))]
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let setup_fn = |app_root: &Path, event_loop: &EventLoop<()>| {
        let display_config_path = app_root.join("config").join("display.ron");

        let display_config = DisplayConfig::load(display_config_path)?;

        let binding_path = app_root.join("config").join("bindings.ron");
        let bindings = <Bindings<StringBindings> as Config>::load(binding_path)?;

        let rendering_bundle = RenderingBundle::<DefaultBackend>::new(display_config, event_loop);

        Ok((bindings, rendering_bundle))
    };

    entry::run_application(setup_fn)
}

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[allow(unused)]
#[cfg(feature = "wasm")]
fn main() {}

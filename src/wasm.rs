#[cfg(feature = "wasm")]
mod wasm {
    use std::path::Path;

    use crate::entry;
    use amethyst::{
        config::Config,
        input::{Axis, Bindings, Button, StringBindings},
        renderer::{types::DefaultBackend, RenderingBundle},
        window::{DisplayConfig, EventLoop},
        winit::event::VirtualKeyCode,
    };
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    #[wasm_bindgen]
    #[derive(Debug, Default)]
    pub struct TowerDefenceBuilder {
        canvas_element: Option<HtmlCanvasElement>,
        input_bindings_str: Option<String>,
    }

    #[wasm_bindgen]
    impl TowerDefenceBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
            self.canvas_element = Some(canvas);
            self
        }

        pub fn with_input_bindings(mut self, input_bindings_str: String) -> Self {
            self.input_bindings_str = Some(input_bindings_str);
            self
        }

        pub fn run(self) {
            crate::init_panic_hook();

            amethyst::start_logger(Default::default());

            let dimensions = self
                .canvas_element
                .as_ref()
                .map(|canvas_element| (canvas_element.width(), canvas_element.height()));

            let display_config = DisplayConfig {
                dimensions,
                ..Default::default()
            };

            let bindings = self
                .input_bindings_str
                .as_ref()
                .map(|input_bindings_str| {
                    <Bindings<StringBindings> as Config>::load_bytes(input_bindings_str.as_bytes())
                        .expect("Failed to deserialize input bindings.")
                })
                .expect("could not find bindings");

            let setup_fn = move |_: &Path, event_loop: &EventLoop<()>| {
                let rendering_bundle = RenderingBundle::<DefaultBackend>::new(
                    display_config,
                    event_loop,
                    self.canvas_element,
                );

                Ok((bindings, rendering_bundle))
            };

            let res = entry::run_application(setup_fn);
            match res {
                Ok(_) => println!("Exited without error"),
                Err(e) => println!("Main returned an error: {:?}", e),
            }
        }
    }
}

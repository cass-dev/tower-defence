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
        /// User supplied canvas, if any.
        canvas_element: Option<HtmlCanvasElement>,
        /// Input bindings data.
        input_bindings_str: Option<String>,
    }

    #[wasm_bindgen]
    impl TowerDefenceBuilder {
        /// Returns a new `MyAppBuilder`.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the canvas element for the `MyAppBuilder`.
        pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
            self.canvas_element = Some(canvas);
            self
        }

        /// Sets the canvas element for the `MyAppBuilder`.
        pub fn with_input_bindings(mut self, input_bindings_str: String) -> Self {
            self.input_bindings_str = Some(input_bindings_str);
            self
        }

        pub fn run(self) {
            // Make panic return a stack trace
            crate::init_panic_hook();

            amethyst::start_logger(Default::default());

            println!("canvas element: {:?}", self.canvas_element);

            let dimensions = self
                .canvas_element
                .as_ref()
                .map(|canvas_element| (canvas_element.width(), canvas_element.height()));
            println!("dimensions: {:?}", dimensions);

            let display_config = DisplayConfig {
                dimensions,
                ..Default::default()
            };

            let bindings = if let Some(input_bindings_str) = self.input_bindings_str.as_ref() {
                <Bindings<StringBindings> as Config>::load_bytes(input_bindings_str.as_bytes())
                    .expect("Failed to deserialize input bindings.")
            } else {
                let mut bindings = Bindings::<StringBindings>::new();
                // let left_paddle_axis = Axis::Emulated {
                //     pos: Button::Key(VirtualKeyCode::W),
                //     neg: Button::Key(VirtualKeyCode::S),
                // };
                // let _ = bindings.insert_axis("left_paddle", left_paddle_axis);
                // let right_paddle_axis = Axis::Emulated {
                //     pos: Button::Key(VirtualKeyCode::Up),
                //     neg: Button::Key(VirtualKeyCode::Down),
                // };
                // let _ = bindings.insert_axis("right_paddle", right_paddle_axis);

                bindings.insert_action_binding(
                    "debug_toggle".to_owned(),
                    vec![Button::Key(VirtualKeyCode::D)].into_iter(),
                );

                bindings
            };

            // The `application_root_dir` parameter is not used as WASM
            // applications don't access the file system.
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

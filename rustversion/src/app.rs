use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::graphics::Graphics;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    graphics: Option<Graphics>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Title")
                        .with_inner_size(winit::dpi::LogicalSize::new(800, 600)),
                )
                .unwrap(),
        );

        let graphics = Graphics::new(window.clone());

        window.request_redraw();

        self.window = Some(window);
        self.graphics = Some(graphics);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = &self.window else {
            return;
        };

        if window_id != window.id() {
            return;
        }

        let Some(graphics) = &mut self.graphics else {
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                graphics.resize(size.width, size.height);
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                graphics.render();
            }

            _ => {}
        }
    }
}

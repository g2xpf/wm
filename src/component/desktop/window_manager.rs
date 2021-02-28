use std::collections::HashMap;

use super::application::Application;
use super::window::Window;
use crate::custom_event::EventProxy;
use crate::Component;
use crate::Global;

pub type WindowId = usize;

pub struct WindowManager {
    window_id_counter: WindowId,
    windows: HashMap<WindowId, Window>,
}

impl WindowManager {
    pub fn new() -> Self {
        let window_id_counter = 0usize;
        let windows = HashMap::new();

        WindowManager {
            window_id_counter,
            windows,
        }
    }

    pub fn spawn(&mut self, app: impl Application, global: &Global) {
        let window = Window::new(self.window_id_counter, app, global);
        self.windows.insert(self.window_id_counter, window);
        self.window_id_counter += 1;
    }
}

impl Component for WindowManager {
    fn draw(&self, proxy: &mut crate::RenderContextProxy) {
        for window in self.windows.values() {
            window.draw(proxy);
        }
    }

    fn update(&mut self) {
        for window in self.windows.values_mut() {
            window.update();
        }
    }

    fn handle_event(&mut self, event: EventProxy, global: &Global) {
        for window in self.windows.values_mut() {
            window.handle_event(event, global)
        }
    }
}

use crate::component::Component;
use crate::custom_event::CustomEvent;
use crate::Global;
use crate::RenderContextProxy;

use glium::glutin::event::Event;
use glium::Surface;

pub mod window;

pub mod application;
use application::Sample;

pub mod window_manager;
use window_manager::WindowManager;

pub struct Desktop {
    window_manager: WindowManager,
}

impl Desktop {
    pub fn new(global: &Global) -> Self {
        let mut window_manager = WindowManager::new();
        let sample = Sample::new(global);
        window_manager.spawn(sample, global);
        Desktop { window_manager }
    }
}

impl Component for Desktop {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        let frame = proxy.frame();
        frame.clear_color(0.8, 0.9, 1.0, 1.0);

        self.window_manager.draw(proxy);
    }

    fn update(&mut self, global: &Global) {
        self.window_manager.update(global);
    }

    fn handle_event(&mut self, event: &Event<'_, CustomEvent>, global: &Global) {
        self.window_manager.handle_event(event, global);
    }
}

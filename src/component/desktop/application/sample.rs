use nalgebra::Vector4;

use crate::component::utils::Input;
use crate::component::Layout;
use crate::custom_event::CustomEvent;
use crate::Global;
use crate::RenderContextProxy;

use super::{AppInfo, Application};
use crate::Component;

use glium::glutin::event::Event;

pub struct Sample {
    app_info: AppInfo,
    input: Input,
}

impl Sample {
    pub fn new(global: &Global) -> Self {
        let title = String::from("App");
        let app_info = AppInfo {
            title,
            layout: Layout::default(),
        };
        let mut input = Input::new(global);
        input.text.set_font_size(16.0);
        input.text.color = Vector4::new(0.1, 0.1, 0.1, 1.0);
        input.background.color = Vector4::new(1.0, 1.0, 1.0, 1.0);
        input.background.round_radius = 2.0;

        Sample { app_info, input }
    }
}

impl Component for Sample {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        self.input.draw(proxy);
    }

    fn set_layout(&mut self, layout: Layout) {
        self.app_info.layout = layout;
        self.input.set_layout(layout);
    }

    fn handle_event(&mut self, event: &Event<'_, CustomEvent>, global: &Global) {
        self.input.handle_event(event, global);
    }

    fn update(&mut self) {
        self.input.update();
    }
}

impl Application for Sample {
    fn get_app_info(&self) -> &AppInfo {
        &self.app_info
    }
}

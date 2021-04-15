use nalgebra::Vector4;

use crate::component::utils::{Button, Input};
use crate::component::Layout;
use crate::custom_event::CustomEvent;
use crate::Global;
use crate::RenderContextProxy;

use super::{AppInfo, Application};
use crate::Component;

use glium::glutin::event::Event;

use nalgebra::Vector2;

pub struct Sample {
    app_info: AppInfo,
    input: Input,
    button: Button,
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

        let mut button = Button::new(global);
        button.set_font_size(16.0);
        button.text.color = Vector4::new(0.0, 0.4, 0.4, 1.0);
        button.text.content = "push!!!!!!!!!!!!!!!!".to_owned();
        button.round_radius = 2.0;

        Sample {
            app_info,
            input,
            button,
        }
    }
}

impl Component for Sample {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        self.input.draw(proxy);
        self.button.draw(proxy);
    }

    fn set_layout(&mut self, layout: Layout) {
        self.app_info.layout = layout;
        let input_ratio = 0.9;
        let input_height = layout.size.y * input_ratio;
        let button_height = layout.size.y - input_height;
        let input_layout = Layout {
            position: layout.position,
            size: Vector2::new(layout.size.x, input_height),
        };
        let button_layout = Layout {
            position: Vector2::new(layout.position.x, layout.position.y + input_height),
            size: Vector2::new(layout.size.x, button_height),
        };
        self.input.set_layout(input_layout);
        self.button.set_layout(button_layout);
    }

    fn handle_event(&mut self, event: &Event<'_, CustomEvent>, global: &Global) {
        self.input.handle_event(event, global);
        self.button.handle_event(event, global);
    }

    fn update(&mut self, global: &Global) {
        self.input.update(global);
        self.button.update(global);
    }
}

impl Application for Sample {
    fn get_app_info(&self) -> &AppInfo {
        &self.app_info
    }
}

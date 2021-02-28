use nalgebra::Vector2;
use nalgebra::Vector4;

use super::{application::Application, window_manager::WindowId};
use crate::component::{Layout, Plane, Text};
use crate::custom_event::EventProxy;
use crate::Component;
use crate::Global;
use crate::RenderContextProxy;

pub struct Window {
    id: WindowId,
    app: Box<dyn Application>,
    title_text: Text,
    background: Plane,
}

impl Window {
    const TITLE_HEIGHT: f32 = 16.0;
    const FRAME_WIDTH: f32 = 5.0;

    pub fn new(id: WindowId, app: impl Application, global: &Global) -> Self {
        let app_info = app.get_app_info();

        let mut title_text = Text::new(&global);
        title_text.set_font_size(Self::TITLE_HEIGHT);
        title_text.color = Vector4::new(0.9, 0.9, 0.9, 1.0);
        title_text.content = app_info.title.clone();

        let app = Box::new(app);

        let mut background = Plane::new(global);
        background.color = Vector4::new(0.2, 0.2, 0.2, 1.0);
        background.round_radius = 3.0;

        let mut window = Window {
            id,
            background,
            app,
            title_text,
        };
        window.set_layout(Layout {
            position: Vector2::new(50.0, 50.0),
            size: Vector2::new(200.0, 480.0),
        });
        window
    }
}

impl Component for Window {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        self.background.draw(proxy);
        self.title_text.draw(proxy);
        self.app.draw(proxy);
    }

    fn update(&mut self) {
        self.background.update();
        self.title_text.update();
        self.app.update();
    }

    fn handle_event(&mut self, event: EventProxy, global: &Global) {
        self.background.handle_event(event, global);
        self.title_text.handle_event(event, global);
        self.app.handle_event(event, global);
    }

    fn set_layout(&mut self, layout: Layout) {
        let title_text_layout = {
            let position = layout.position + Vector2::new(Self::FRAME_WIDTH, Self::FRAME_WIDTH);
            let size = Vector2::new(layout.size.x - Self::FRAME_WIDTH * 2.0, Self::TITLE_HEIGHT);
            Layout { position, size }
        };
        let app_layout = {
            let position = layout.position
                + Vector2::new(
                    Self::FRAME_WIDTH,
                    Self::TITLE_HEIGHT + Self::FRAME_WIDTH * 2.0,
                );
            let size = Vector2::new(
                layout.size.x - Self::FRAME_WIDTH * 2.0,
                layout.size.y - Self::TITLE_HEIGHT - Self::FRAME_WIDTH * 3.0,
            );
            Layout { position, size }
        };

        self.background.set_layout(layout);
        self.title_text.set_layout(title_text_layout);
        self.app.set_layout(app_layout);
    }
}

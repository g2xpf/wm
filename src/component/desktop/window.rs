use nalgebra::Vector2;
use nalgebra::Vector4;

use super::{application::Application, window_manager::WindowId};
use crate::component::{Layout, Plane, Text};
use crate::custom_event::EventProxy;
use crate::Component;
use crate::Global;
use crate::RenderContextProxy;

use glium::glutin::{
    dpi::LogicalPosition,
    event::{ElementState, Event, WindowEvent},
};

pub struct Window {
    id: WindowId,
    app: Box<dyn Application>,
    title_text: Text,
    background: Plane,
    layout: Layout,
    dragging_state: Option<LogicalPosition<f64>>,
}

impl Window {
    const TITLE_HEIGHT: f32 = 16.0;
    const FRAME_WIDTH: f32 = 5.0;

    pub fn new(id: WindowId, app: impl Application, global: &Global) -> Self {
        let app_info = app.get_app_info();

        let mut title_text = Text::new(&global);
        title_text.set_font_size(Self::TITLE_HEIGHT);
        title_text.color = Vector4::new(0.4, 0.7, 0.9, 1.0);
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
            layout: Layout::default(),
            dragging_state: None,
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

    fn update(&mut self, global: &Global) {
        if let Some(position) = &mut self.dragging_state {
            let new_position = global.cursor_position();
            let (dx, dy) = (new_position.x - position.x, new_position.y - position.y);
            *position = new_position;
            let mut layout = self.layout;
            layout.position.x += dx as f32;
            layout.position.y += dy as f32;
            self.set_layout(layout);
            global.request_redraw();
        }
        self.background.update(global);
        self.title_text.update(global);
        self.app.update(global);
    }

    fn handle_event(&mut self, event: EventProxy, global: &Global) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    ..
                } => {
                    let cursor_position = global.cursor_position();
                    if self.title_text.layout.contains(&cursor_position) {
                        self.dragging_state = Some(cursor_position);
                    }
                }
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    self.dragging_state = None;
                }
                _ => {}
            },
            _ => {}
        }

        self.background.handle_event(event, global);
        self.title_text.handle_event(event, global);
        self.app.handle_event(event, global);
    }

    fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
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

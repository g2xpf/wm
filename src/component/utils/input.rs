use super::{Plane, Text};
use crate::custom_event::EventProxy;
use crate::Global;
use crate::RenderContextProxy;
use crate::{component::Layout, Component};

use glium::glutin;
use glutin::event::ElementState;
use glutin::event::Event;
use glutin::event::WindowEvent;

use nalgebra::Vector4;

#[derive(Clone, Copy)]
struct Vertex {}

pub struct Input {
    pub text: Text,
    pub background: Plane,
    focus: bool,
}

impl Input {
    pub fn new(global: &Global) -> Self {
        let mut text = Text::new_cursored(global);
        text.content = String::new();

        let mut background = Plane::new(global);
        background.color = Vector4::new(0.4, 0.9, 0.8, 1.0);
        background.request_redraw();
        let focus = false;

        Input {
            text,
            background,
            focus,
        }
    }

    fn is_cursor_hovering(&self, global: &Global) -> bool {
        self.background.layout.contains(&global.cursor_position())
    }
}

impl Component for Input {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        self.background.draw(proxy);
        self.text.draw(proxy);
    }

    fn update(&mut self, global: &Global) {
        self.background.update(global);
        self.text.update(global);
    }

    fn handle_event(&mut self, event: EventProxy, global: &Global) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    ..
                } => {
                    let is_cursor_hovering = self.is_cursor_hovering(global);
                    if is_cursor_hovering && !self.focus {
                        self.focus = true;
                        self.text.set_cursor_visibility(true);
                        global.request_redraw();
                    } else if !is_cursor_hovering && self.focus {
                        self.focus = false;
                        self.text.set_cursor_visibility(false);
                        global.request_redraw();
                    }
                }
                WindowEvent::ReceivedCharacter(c) if self.focus => match *c {
                    '\u{8}' | '\u{7f}' => {
                        self.text.content.pop();
                    }
                    c => {
                        self.text.content.push(c);
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn set_layout(&mut self, layout: Layout) {
        self.background.set_layout(layout);
        self.text.set_layout(layout);
    }
}

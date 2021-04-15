use std::rc::Rc;

use crate::{custom_event::CustomEvent, RenderContext};

use glium::glutin::event_loop::EventLoopClosed;
use glium::glutin::{
    dpi::{LogicalPosition, PhysicalPosition},
    event::Event,
    event_loop::EventLoopProxy,
};
use glium::Display;

pub struct Global {
    pub font: Rc<rusttype::Font<'static>>,
    pub render_context: RenderContext<'static>,
    cursor_position: PhysicalPosition<f64>,
    pub scale_factor: f64,
    pub event_loop_proxy: EventLoopProxy<CustomEvent>,
}

impl Global {
    pub fn new(
        font: rusttype::Font<'static>,
        render_context: RenderContext<'static>,
        event_loop_proxy: EventLoopProxy<CustomEvent>,
    ) -> Self {
        let font = Rc::new(font);
        let scale_factor = render_context.scale_factor();
        let cursor_position = PhysicalPosition::new(0.0, 0.0);

        Global {
            font,
            render_context,
            scale_factor,
            cursor_position,
            event_loop_proxy,
        }
    }

    pub fn send_event(&self, event: CustomEvent) -> Result<(), EventLoopClosed<CustomEvent>> {
        self.event_loop_proxy.send_event(event)
    }

    pub fn handle_event(&mut self, event: &Event<'_, CustomEvent>) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    self.cursor_position = *position;
                }
                glium::glutin::event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    self.scale_factor = *scale_factor;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn cursor_position(&self) -> LogicalPosition<f64> {
        self.cursor_position.to_logical(self.scale_factor)
    }

    pub fn request_redraw(&self) {
        self.display().gl_window().window().request_redraw();
    }

    pub fn display(&self) -> &Display {
        self.render_context.display()
    }
}

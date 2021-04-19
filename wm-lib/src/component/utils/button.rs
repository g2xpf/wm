use super::Text;
use crate::RenderContextProxy;
use crate::{component::Layout, custom_event::CustomEvent};

use glium::{
    draw_parameters::DrawParameters,
    glutin::event::WindowEvent,
    glutin::event::{ElementState, Event},
    implement_vertex,
    index::NoIndices,
    index::PrimitiveType,
    uniform, Display, Program, Surface, VertexBuffer,
};
use nalgebra::Vector2;

use crate::Component;
use crate::Global;

#[derive(Copy, Clone)]
pub struct Vertex {
    a_position: [f32; 2],
}

implement_vertex!(Vertex, a_position);

pub struct Button {
    pub text: Text,
    pub layout: Layout,
    pub visibility: bool,
    pub on_click: Option<Box<dyn Fn() + 'static>>,
    pub round_radius: f32,

    pressed: bool,
    should_redraw: bool,
    display: Display,
    program: Program,
    vbo: Option<VertexBuffer<Vertex>>,
}

impl Button {
    const VSRC: &'static str = include_str!("button/button.vert");
    const FSRC: &'static str = include_str!("button/button.frag");
    const FRAME_WIDTH: f32 = 3.0;

    pub fn new(global: &Global) -> Self {
        let text = Text::new(global);
        let display = global.render_context.display().clone();
        let program = Program::from_source(global.display(), Self::VSRC, Self::FSRC, None).unwrap();
        let vbo = None;
        let round_radius = 10.0;

        Button {
            text,
            layout: Layout::default(),
            visibility: true,
            on_click: None,
            round_radius,

            pressed: false,
            should_redraw: true,
            display,
            program,
            vbo,
        }
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.text.set_font_size(font_size);
        self.text.update_vbo();
    }

    pub fn update_vbo(&mut self) {
        if !self.should_redraw {
            return;
        }
        let left = 0.0;
        let right = self.layout.size.x;
        let down = 0.0;
        let up = self.layout.size.y;
        let vbo = VertexBuffer::new(
            &self.display,
            &[
                Vertex {
                    a_position: [left, down],
                },
                Vertex {
                    a_position: [right, down],
                },
                Vertex {
                    a_position: [right, up],
                },
                Vertex {
                    a_position: [left, up],
                },
            ],
        )
        .expect("failed to create vbo");
        self.vbo = Some(vbo);
    }

    fn is_cursor_hovering(&self, global: &Global) -> bool {
        self.layout.contains(&global.cursor_position())
    }
}

impl Component for Button {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        if self.visibility {
            let center: [f32; 2] = (self.layout.position + 0.5 * self.layout.size).into();
            let scale_factor = proxy.scale_factor() as f32;
            let resolution: [f32; 2] = proxy.frame_buffer_size().into();
            let frame = proxy.frame();
            let position: [f32; 2] = self.layout.position.into();
            let button_size: [f32; 2] = self.layout.size.into();
            frame
                .draw(
                    self.vbo.as_ref().expect("vbo not initialized"),
                    NoIndices(PrimitiveType::TriangleFan),
                    &self.program,
                    &uniform! {
                        u_pressed: self.pressed,
                        u_center_position: center,
                        u_scale_factor: scale_factor,
                        u_position: position,
                        u_button_size: button_size,
                        u_resolution: resolution,
                        u_round_radius: self.round_radius,
                        u_color: [1.0f32, 1.0, 0.5, 1.0],
                    },
                    &DrawParameters::default(),
                )
                .expect("failed to draw");
        }
        self.text.draw(proxy);
    }

    fn handle_event(&mut self, event: &Event<'_, CustomEvent>, global: &Global) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::MouseInput { state, .. } => match state {
                    ElementState::Pressed if self.is_cursor_hovering(global) => {
                        if let Some(callback) = &self.on_click {
                            callback();
                        }

                        self.pressed = true;
                        global.request_redraw()
                    }
                    ElementState::Released => {
                        self.pressed = false;
                        global.request_redraw();
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, global: &Global) {
        self.update_vbo();
        self.text.update(global);
    }

    fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        let text_layout = Layout {
            position: layout.position + Vector2::new(Self::FRAME_WIDTH, Self::FRAME_WIDTH),
            size: layout.size - 2.0 * Vector2::new(Self::FRAME_WIDTH, Self::FRAME_WIDTH),
        };
        self.text.set_layout(text_layout);
    }
}

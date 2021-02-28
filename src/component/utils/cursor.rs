use crate::component::Layout;
use crate::RenderContextProxy;

use glium::{
    draw_parameters::Blend, draw_parameters::DrawParameters, implement_vertex, index::IndexType,
    index::NoIndices, index::PrimitiveType, uniform, Display, Program, Surface, VertexBuffer,
};
use nalgebra::{Vector2, Vector4};

use crate::Component;
use crate::Global;

#[derive(Copy, Clone)]
pub struct Vertex {
    a_position: [f32; 2],
}

implement_vertex!(Vertex, a_position);

#[derive(Copy, Clone)]
pub enum CursorShape {
    Line,
    Box,
}

pub struct Cursor {
    pub shape: CursorShape,
    pub font_size: f32,
    pub local_position: Vector2<f32>,
    pub color: Vector4<f32>,
    pub layout: Layout,
    pub visibility: bool,

    display: Display,
    program: Program,
    vbo: Option<VertexBuffer<Vertex>>,
}

impl Cursor {
    const VSRC: &'static str = include_str!("cursor/cursor.vert");
    const FSRC: &'static str = include_str!("cursor/cursor.frag");

    pub fn new(global: &Global) -> Self {
        let display = global.render_context.display().clone();
        let shape = CursorShape::Line;
        let program = Program::from_source(global.display(), Self::VSRC, Self::FSRC, None)
            .unwrap_or_else(|err| panic!(format!("{:#?}", err)));
        let vbo = None;
        let color = Vector4::new(0.0, 0.0, 0.0, 1.0);

        Cursor {
            display,
            shape,
            program,
            font_size: 24.0,
            vbo,
            color,
            local_position: Vector2::new(0.0, 0.0),
            visibility: false,
            layout: Layout::default(),
        }
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.update_vbo();
    }

    pub fn update_vbo(&mut self) {
        let left = 0.0;
        let right = self.font_size * 0.75;
        let down = 0.0;
        let up = self.font_size;
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
}

impl Component for Cursor {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        if !self.visibility {
            return;
        }
        let scale_factor = proxy.scale_factor() as f32;
        let resolution: [f32; 2] = proxy.frame_buffer_size().into();
        let frame = proxy.frame();
        let position: [f32; 2] = (self.local_position + scale_factor * self.layout.position).into();
        let color: [f32; 4] = self.color.into();
        frame
            .draw(
                self.vbo.as_ref().expect("vbo not initialized"),
                NoIndices(PrimitiveType::TriangleFan),
                &self.program,
                &uniform! {
                    u_font_size: self.font_size,
                    u_scale_factor: scale_factor,
                    u_position: position,
                    u_resolution: resolution,
                    u_cursor_type: self.shape as i32,
                    u_color: color
                },
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..DrawParameters::default()
                },
            )
            .expect("failed to draw");
    }

    fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }
}

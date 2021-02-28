use nalgebra::Vector4;

use crate::component::Layout;
use crate::Component;
use crate::Global;
use crate::RenderContextProxy;

use glium::Display;
use glium::DrawParameters;
use glium::Program;
use glium::VertexBuffer;
use glium::{implement_vertex, index::NoIndices, Surface};
use glium::{index::PrimitiveType, uniform};

#[derive(Clone, Copy)]
struct Vertex {
    a_position: [f32; 2],
}
implement_vertex!(Vertex, a_position);

pub struct Plane {
    pub layout: Layout,
    pub color: Vector4<f32>,
    pub round_radius: f32,

    should_redraw: bool,
    display: Display,
    program: Program,
    vbo: Option<VertexBuffer<Vertex>>,
}

impl Plane {
    const VSRC: &'static str = include_str!("plane/plane.vert");
    const FSRC: &'static str = include_str!("plane/plane.frag");

    pub fn new(global: &Global) -> Self {
        let display = global.display().clone();
        let program = Program::from_source(&display, Self::VSRC, Self::FSRC, None)
            .unwrap_or_else(|err| panic!(format!("{:#?}", err)));
        let vbo = None;

        Plane {
            program,
            vbo,
            display,
            should_redraw: true,
            layout: Layout::default(),
            color: Vector4::new(0.8, 0.8, 0.8, 0.8),
            round_radius: 10.0,
        }
    }

    pub fn update_vbo(&mut self) {
        if self.should_redraw {
            self.should_redraw = false;
            let left = 0.0;
            let right = self.layout.size.x;
            let bottom = 0.0;
            let top = self.layout.size.y;

            let vertex_buffer = VertexBuffer::new(
                &self.display,
                &[
                    Vertex {
                        a_position: [left, bottom],
                    },
                    Vertex {
                        a_position: [right, bottom],
                    },
                    Vertex {
                        a_position: [right, top],
                    },
                    Vertex {
                        a_position: [left, top],
                    },
                ],
            )
            .expect("failed to create vbo");
            self.vbo = Some(vertex_buffer);
            self.display.gl_window().window().request_redraw()
        }
    }

    pub fn request_redraw(&mut self) {
        self.should_redraw = true;
    }
}

impl Component for Plane {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        let center: [f32; 2] = (self.layout.position + 0.5 * self.layout.size).into();
        let plane_size: [f32; 2] = self.layout.size.into();
        let resolution: [f32; 2] = proxy.frame_buffer_size().into();
        let background_color: [f32; 4] = self.color.into();
        let scale_factor = self.display.gl_window().window().scale_factor() as f32;
        let frame = proxy.frame();
        let position: [f32; 2] = self.layout.position.into();

        frame
            .draw(
                self.vbo.as_ref().expect("vbo not initialized"),
                NoIndices(PrimitiveType::TriangleFan),
                &self.program,
                &uniform! {
                    u_center_position: center,
                    u_color: background_color,
                    u_round_radius: self.round_radius,
                    u_plane_size: plane_size,
                    u_resolution: resolution,
                    u_scale_factor: scale_factor,
                    u_position: position
                },
                &DrawParameters::default(),
            )
            .expect("failed to draw");
    }

    fn update(&mut self, _global: &Global) {
        self.update_vbo();
    }

    fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }
}

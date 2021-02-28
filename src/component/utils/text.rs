use crate::Global;
use crate::RenderContextProxy;
use crate::{component::Layout, Component};

use glium::Surface;
use glium::VertexBuffer;
use glium::{index::NoIndices, program::Program};
use glium::{uniform, DrawParameters};
use glium::{uniforms::MagnifySamplerFilter, Display};
use nalgebra::Vector4;

use std::ops::{Deref, DerefMut};

mod raw_text;

use raw_text::FontRenderInfo;
use raw_text::RawText;

pub struct Text {
    display: Display,

    inner: RawText<'static>,
    inner_edited: bool,

    program: Program,
    vbo: Option<VertexBuffer<FontRenderInfo>>,

    pub color: Vector4<f32>,
    pub layout: Layout,
}

impl Text {
    const VSRC: &'static str = include_str!("text/text.vert");
    const FSRC: &'static str = include_str!("text/text.frag");

    fn from_raw_text(raw_text: RawText<'static>, global: &Global) -> Self {
        let display = global.display().clone();
        let program = Program::from_source(&display, Self::VSRC, Self::FSRC, None)
            .unwrap_or_else(|err| panic!(format!("{:#?}", err)));
        let vbo = None;
        let color = Vector4::new(0.0, 0.0, 0.0, 1.0);

        Self {
            display,
            inner: raw_text,
            inner_edited: false,
            program,
            vbo,
            color,
            layout: Layout::default(),
        }
    }

    pub fn new(global: &Global) -> Self {
        let raw_text = RawText::from_internal(global.display(), &global.font);
        Self::from_raw_text(raw_text, global)
    }

    pub fn new_cursored(global: &Global) -> Self {
        let raw_text = RawText::from_internal(global.display(), &global.font).with_cursor(global);
        Self::from_raw_text(raw_text, global)
    }

    pub fn set_cursor_visibility(&mut self, visibility: bool) -> bool {
        self.inner.set_cursor_visibility(visibility)
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.inner.set_font_size(font_size);
    }

    pub fn update_vbo(&mut self) {
        if self.inner_edited {
            self.inner_edited = false;
            self.inner.update_cache();
            let render_info = self.inner.to_font_render_info();
            self.vbo = Some(
                VertexBuffer::new(&self.display, &render_info)
                    .expect("failed to create vertex buffer"),
            );
            self.display.gl_window().window().request_redraw();
        }
    }

    pub fn request_redraw(&mut self) {
        self.inner_edited = true;
    }
}

impl Component for Text {
    fn draw(&self, proxy: &mut RenderContextProxy) {
        let vbo = self.vbo.as_ref().expect("vbo not initialized");
        let resolution: [f32; 2] = proxy.frame_buffer_size().into();
        let scale_factor = proxy.scale_factor() as f32;
        let position = self.layout.position;
        let color: [f32; 4] = self.color.into();

        proxy
            .frame()
            .draw(
                vbo,
                NoIndices(glium::index::PrimitiveType::TrianglesList),
                &self.program,
                &uniform! {
                    u_resolution: resolution,
                    u_color: color,
                    u_glyph_texture: self.inner.create_texture(MagnifySamplerFilter::Linear),
                    u_scale_factor: scale_factor,
                    u_position: [position.x, position.y],
                },
                &DrawParameters {
                    multisampling: true,
                    ..DrawParameters::default()
                },
            )
            .expect("failed to draw text");
        if let Some(cursor) = self.inner.cursor.as_ref() {
            cursor.draw(proxy);
        }
    }

    fn update(&mut self, _global: &Global) {
        self.update_vbo();
    }

    fn set_layout(&mut self, layout: Layout) {
        self.inner.set_wrap_bound(layout.size.x as u32);
        self.layout = layout;
        if let Some(cursor) = self.inner.cursor.as_mut() {
            cursor.set_layout(layout);
        }
    }
}

impl Deref for Text {
    type Target = RawText<'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_edited = true;
        &mut self.inner
    }
}

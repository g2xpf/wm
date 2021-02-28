use glium::implement_vertex;
use glium::texture::{ClientFormat, MipmapsOption, RawImage2d};
use glium::texture::{SrgbFormat, SrgbTexture2d};
use glium::uniforms::{MagnifySamplerFilter, Sampler};
use glium::Display;
use glium::Rect;

use rusttype::gpu_cache::Cache;
use rusttype::Font;
use rusttype::{point, PositionedGlyph, Scale};

use unicode_normalization::UnicodeNormalization;

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::rc::Rc;

use crate::component::utils::Cursor;
use crate::Global;

use nalgebra::Vector2;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct FontRenderInfo {
    pub a_uv: [f32; 2],
    pub a_position: [i32; 2],
}
implement_vertex!(FontRenderInfo, a_uv, a_position);

pub struct RawText<'a> {
    pub content: String,
    font_size: f32,
    pub(super) cursor: Option<Cursor>,
    display: Display,
    cache: Cache<'a>,
    glyphs: Vec<PositionedGlyph<'a>>,
    font: Rc<Font<'a>>,
    cache_tex: SrgbTexture2d,
    wrap_bound: u32,
}

impl<'a> RawText<'a> {
    pub(super) fn from_bytes(display: &Display, bytes: &'a [u8]) -> Self {
        let font = Font::try_from_bytes(bytes).expect("failed to generate font");
        let font = Rc::new(font);
        Self::from_internal(display, &font)
    }

    pub(super) fn from_path(
        display: &Display,
        path: impl AsRef<Path>,
    ) -> io::Result<RawText<'static>> {
        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let font = rusttype::Font::try_from_vec(buf).expect("failed to generate font");
        let font = Rc::new(font);
        Ok(Self::from_internal(display, &font))
    }

    pub(super) fn from_internal<'font>(
        display: &Display,
        font: &Rc<Font<'font>>,
    ) -> RawText<'font> {
        let inner_size = display.gl_window().window().inner_size();
        let wrap_bound = inner_size.width;
        let (cache_width, cache_height) = (inner_size.width, inner_size.height);

        let display = display.clone();
        let cache = Cache::builder()
            .dimensions(cache_width, cache_height)
            .build();
        let cache_tex = SrgbTexture2d::with_format(
            &display,
            RawImage2d {
                data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                width: cache_width,
                height: cache_height,
                format: ClientFormat::U8,
            },
            SrgbFormat::U8U8U8U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap();
        let font = Rc::clone(font);
        let glyphs = Vec::new();
        let text = String::new();

        RawText {
            display,
            cursor: None,
            cache,
            cache_tex,
            font,
            glyphs,
            content: text,
            wrap_bound,
            font_size: 24.0,
        }
    }

    pub fn with_cursor(mut self, global: &Global) -> Self {
        self.cursor = Some(Cursor::new(global));
        self
    }

    pub fn set_cursor_visibility(&mut self, visibility: bool) -> bool {
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.visibility = visibility;
            true
        } else {
            false
        }
    }

    pub(super) fn set_wrap_bound(&mut self, bound: u32) {
        let bound = bound as f64;
        let scale_factor = self.display.gl_window().window().scale_factor();
        self.wrap_bound = (bound * scale_factor) as u32;
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.set_font_size(font_size);
        }
    }

    pub(super) fn update_cache(&mut self) {
        let scale_factor = self.display.gl_window().window().scale_factor();
        let scale = Scale::uniform(self.font_size * scale_factor as f32);
        let mut glyphs = Vec::new();
        let v_metrics = self.font.v_metrics(scale);
        let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        let mut caret = point(0.0, v_metrics.ascent);
        let mut last_glyph_id = None;

        for c in self.content.nfc() {
            if c.is_control() {
                match c {
                    '\r' => {
                        caret = point(0.0, caret.y + advance_height);
                    }
                    '\n' => {}
                    _ => {}
                }
                continue;
            }
            let base_glyph = self.font.glyph(c);
            if let Some(id) = last_glyph_id.take() {
                caret.x += self.font.pair_kerning(scale, id, base_glyph.id());
            }
            last_glyph_id = Some(base_glyph.id());
            let mut glyph = base_glyph.scaled(scale).positioned(caret);
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.max.x > self.wrap_bound as i32 {
                    caret = point(0.0, caret.y + advance_height);
                    glyph.set_position(caret);
                    last_glyph_id = None;
                }
            }
            caret.x += glyph.unpositioned().h_metrics().advance_width;

            self.cache.queue_glyph(0, glyph.clone());
            glyphs.push(glyph);
        }
        self.glyphs = glyphs;

        // separate ownership
        let cache = &mut self.cache;
        let cache_tex = &mut self.cache_tex;
        cache
            .cache_queued(|rect, data| {
                cache_tex.main_level().write(
                    Rect {
                        left: rect.min.x,
                        bottom: rect.min.y,
                        width: rect.width(),
                        height: rect.height(),
                    },
                    RawImage2d {
                        data: Cow::Borrowed(data),
                        width: rect.width(),
                        height: rect.height(),
                        format: ClientFormat::U8,
                    },
                )
            })
            .unwrap();
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.local_position = Vector2::new(caret.x, caret.y - v_metrics.ascent);
        }
    }

    pub(super) fn create_texture(
        &self,
        sampler_filter: MagnifySamplerFilter,
    ) -> Sampler<SrgbTexture2d> {
        self.cache_tex.sampled().magnify_filter(sampler_filter)
    }

    pub(super) fn to_font_render_info(&self) -> Vec<FontRenderInfo> {
        let mut render_info = vec![];

        for glyph in &self.glyphs {
            match self.cache.rect_for(0, glyph) {
                Err(_) | Ok(None) => continue,
                Ok(Some(info)) => {
                    let uv_left_bottom = [info.0.min.x, info.0.min.y];
                    let uv_right_bottom = [info.0.max.x, info.0.min.y];
                    let uv_left_top = [info.0.min.x, info.0.max.y];
                    let uv_right_top = [info.0.max.x, info.0.max.y];

                    let position_left_bottom = [info.1.min.x, info.1.min.y];
                    let position_right_bottom = [info.1.max.x, info.1.min.y];
                    let position_left_top = [info.1.min.x, info.1.max.y];
                    let position_right_top = [info.1.max.x, info.1.max.y];

                    render_info.push(FontRenderInfo {
                        a_position: position_left_bottom,
                        a_uv: uv_left_bottom,
                    });
                    render_info.push(FontRenderInfo {
                        a_position: position_right_bottom,
                        a_uv: uv_right_bottom,
                    });
                    render_info.push(FontRenderInfo {
                        a_position: position_right_top,
                        a_uv: uv_right_top,
                    });
                    render_info.push(FontRenderInfo {
                        a_position: position_right_top,
                        a_uv: uv_right_top,
                    });
                    render_info.push(FontRenderInfo {
                        a_position: position_left_top,
                        a_uv: uv_left_top,
                    });
                    render_info.push(FontRenderInfo {
                        a_position: position_left_bottom,
                        a_uv: uv_left_bottom,
                    });
                }
            }
        }

        render_info
    }
}

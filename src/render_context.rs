use glium::Display;
use glium::Frame;
use glium::{
    framebuffer::{DefaultFramebuffer, SimpleFrameBuffer, ToColorAttachment, ValidationError},
    glutin::dpi::PhysicalSize,
};

pub struct RenderContext<'a> {
    rctx: RawRenderContext<'a>,
}

pub struct RenderContextProxy<'a, 'b> {
    rctx: &'b mut RawRenderContext<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn new(display: Display) -> Self {
        let rctx = RawRenderContext::new(display);
        RenderContext { rctx }
    }

    pub fn create_proxy<'b>(&'b mut self) -> RenderContextProxy<'a, 'b> {
        RenderContextProxy::new(&mut self.rctx)
    }

    pub fn display(&self) -> &Display {
        self.rctx.display()
    }

    pub fn scale_factor(&self) -> f64 {
        self.rctx.scale_factor()
    }
}

impl<'a, 'b> RenderContextProxy<'a, 'b> {
    pub fn new(rctx: &'b mut RawRenderContext<'a>) -> Self {
        RenderContextProxy { rctx }
    }

    pub fn frame_buffer_size(&self) -> PhysicalSize<u32> {
        self.rctx.frame_buffer_size()
    }

    pub fn frame(&mut self) -> &mut Frame {
        self.rctx.frame()
    }

    pub fn simple_framebuffer(
        &mut self,
        t: impl ToColorAttachment<'a>,
    ) -> Result<&mut SimpleFrameBuffer<'a>, ValidationError> {
        self.rctx.simple_framebuffer(t)
    }

    pub fn default_framebuffer(&mut self) -> &mut DefaultFramebuffer {
        self.rctx.default_framebuffer()
    }

    pub fn scale_factor(&self) -> f64 {
        self.rctx.scale_factor()
    }
}

impl Drop for RenderContextProxy<'_, '_> {
    fn drop(&mut self) {
        self.rctx.finalize();
    }
}

pub struct RawRenderContext<'a> {
    display: Display,
    frame: Option<Frame>,
    default_framebuffer: Option<DefaultFramebuffer>,
    simple_framebuffer: Option<SimpleFrameBuffer<'a>>,
}

impl<'a> RawRenderContext<'a> {
    pub fn new(display: Display) -> Self {
        RawRenderContext {
            display,
            frame: None,
            default_framebuffer: None,
            simple_framebuffer: None,
        }
    }

    pub fn frame_buffer_size(&self) -> PhysicalSize<u32> {
        self.display().gl_window().window().inner_size()
    }

    pub fn frame(&mut self) -> &mut Frame {
        if let Some(ref mut frame) = self.frame {
            frame
        } else {
            let frame = self.display.draw();
            self.frame = Some(frame);
            self.frame.as_mut().unwrap()
        }
    }

    pub fn default_framebuffer(&mut self) -> &mut DefaultFramebuffer {
        if let Some(ref mut default_framebuffer) = self.default_framebuffer {
            default_framebuffer
        } else {
            let default_framebuffer = DefaultFramebuffer::back_left(&self.display);
            self.default_framebuffer = Some(default_framebuffer);
            self.default_framebuffer.as_mut().unwrap()
        }
    }

    pub fn simple_framebuffer(
        &mut self,
        t: impl ToColorAttachment<'a>,
    ) -> Result<&mut SimpleFrameBuffer<'a>, ValidationError> {
        if let Some(ref mut simple_framebuffer) = self.simple_framebuffer {
            Ok(simple_framebuffer)
        } else {
            let simple_framebuffer = SimpleFrameBuffer::new(&self.display, t)?;
            self.simple_framebuffer = Some(simple_framebuffer);
            Ok(self.simple_framebuffer.as_mut().unwrap())
        }
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn scale_factor(&self) -> f64 {
        self.display.gl_window().window().scale_factor()
    }

    fn finalize(&mut self) {
        if let Some(frame) = self.frame.take() {
            frame.finish().expect("failed to finish frame");
        }
    }
}

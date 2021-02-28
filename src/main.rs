use glium::glutin::{self, dpi::LogicalSize};
use glium::Display;

use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;

use rusttype::Font;

use wm::custom_event::{self, CustomEvent};
use wm::Component;
use wm::Desktop;
use wm::Global;
use wm::RenderContext;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::<CustomEvent>::with_user_event();
    let window_size: LogicalSize<f32> = (960.0, 720.0).into();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("wm")
        .with_inner_size(window_size);
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = Display::new(wb, cb, &event_loop).unwrap();
    let render_context = RenderContext::new(display);

    let font = Font::try_from_bytes(include_bytes!("../resource/GenRyuMinJP-Regular.ttf"))
        .expect("failed to load font");
    let mut global = Global::new(font, render_context);

    let mut desktop = Desktop::new(&global);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        global.handle_event(&event);
        desktop.handle_event(&event, &global);

        match event {
            // handling window events
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },

            Event::UserEvent(custom_event::CustomEvent::WindowEvent(
                custom_event::WindowEvent::Show(window_id),
            )) => {}

            Event::MainEventsCleared => {
                desktop.update();
            }

            // rendering
            Event::RedrawRequested(_) => {
                let mut proxy = global.render_context.create_proxy();
                desktop.draw(&mut proxy);
            }
            _ => {}
        };
    });
}

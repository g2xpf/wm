use crate::component::desktop::window_manager::WindowId;
use glium::glutin::event;

pub type EventProxy<'a> = &'a event::Event<'a, CustomEvent>;

#[derive(Clone)]
pub enum CustomEvent {
    WindowEvent(WindowEvent),
}

#[derive(Clone)]
pub enum WindowEvent {
    Close(WindowId),
    Show(WindowId),
    Hide(WindowId),
}

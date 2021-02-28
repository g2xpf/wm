use crate::component::desktop::window_manager::WindowId;
use glium::glutin::event;

pub type EventProxy<'a> = &'a event::Event<'a, CustomEvent>;

pub enum CustomEvent {
    WindowEvent(WindowEvent),
}

pub enum WindowEvent {
    Close(WindowId),
    Show(WindowId),
    Hide(WindowId),
}

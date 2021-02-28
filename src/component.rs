use super::render_context::RenderContextProxy;
use crate::custom_event::CustomEvent;
use crate::Global;

use glium::glutin::{dpi::LogicalPosition, event::Event};

use nalgebra::Vector2;

pub mod desktop;
mod utils;

pub use desktop::window::Window;
pub use utils::Plane;
pub use utils::Text;

/// Logical component layout
#[derive(Clone, Copy, Default, Debug)]
pub struct Layout {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}

impl Layout {
    pub fn contains(&self, logical_position: &LogicalPosition<f64>) -> bool {
        println!("logical_position: {:?}", logical_position);
        println!("layout: {:?}", *self);
        let (x, y) = (logical_position.x as f32, logical_position.y as f32);
        self.position.x <= x
            && x <= self.position.x + self.size.x
            && self.position.y <= y
            && y <= self.position.y + self.size.y
    }
}

#[allow(unused_variables)]
pub trait Component {
    fn draw(&self, _proxy: &mut RenderContextProxy) {}
    fn handle_event(&mut self, event: &Event<'_, CustomEvent>, global: &Global) {}
    fn update(&mut self) {}

    fn set_layout(&mut self, _layout: Layout) {}
}

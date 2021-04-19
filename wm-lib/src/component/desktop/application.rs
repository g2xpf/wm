use crate::component::Layout;
use crate::rw_cell::Rw;
use crate::Component;

mod sample;
pub use sample::Sample;

pub struct AppInfo {
    pub title: Rw<String>,
    pub layout: Layout,
}

pub trait Application: Component + 'static {
    fn get_app_info(&self) -> &AppInfo;
}

pub use igneous::{IgneousApp, IgneousState};
pub mod igneous;


pub(crate) mod ui_prelude {
    pub use eframe::{Frame, egui};
    pub use egui::{Context, Ui};
}
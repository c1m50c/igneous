use igneous_components::{IgneousApp, IgneousState};
use eframe::NativeOptions;


fn main() {
    let mut igneous_app = IgneousApp::new(IgneousState::Select {
        options: igneous_types::notebook::read_selection_options("notebooks/"),
    });

    let native_options = NativeOptions {
        initial_window_size: Some(egui::Vec2::new(1280.0, 800.0)),
        decorated: true, centered: true,
        .. NativeOptions::default()
    };

    eframe::run_native(
        "Igneous", native_options,
        Box::new(|_| Box::new(igneous_app))
    );
}
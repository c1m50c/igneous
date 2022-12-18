use igneous_types::notebook::{Notebook, SelectionOption};
use ui_prelude::*;


pub enum IgneousState {
    Select { options: Vec<SelectionOption> },
    Unlock { selected: SelectionOption, },
    Editing { notebook: Notebook },
}


pub struct IgneousApp {
    state: IgneousState,
}


impl IgneousApp {
    pub fn new(state: IgneousState) -> Self {
        return Self { state };
    }

    pub fn select_state(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        let IgneousState::Select { options } = &mut self.state else { unreachable!() };
        let mut selected = None;

        // Select Existing Notebook
        ui.vertical_centered_justified(|ui| {
            let mut responses = Vec::with_capacity(options.len());

            for option in options {
                let response = ui.button(option.identifier.as_str());
                responses.push((response, &option));
            }

            let maybe_clicked = responses.iter()
                .find(|(response, _)| response.clicked());

            if let Some((response, option)) = maybe_clicked {
                let notebook = Notebook::from_path(&option.path);
                selected = Some(notebook);
            }
        });

        // Change State
        if let Some(notebook) = selected {
            self.state = IgneousState::Editing { notebook };
        }
    }

    pub fn unlock_state(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        let IgneousState::Unlock { selected } = &mut self.state else { unreachable!() };
    }

    pub fn editing_state(&mut self, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
        let IgneousState::Editing { notebook } = &mut self.state else { unreachable!() };
    }
}


impl eframe::App for IgneousApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                IgneousState::Select { .. } => self.select_state(ctx, frame, ui),
                IgneousState::Unlock { .. } => self.unlock_state(ctx, frame, ui),
                IgneousState::Editing { .. } => self.editing_state(ctx, frame, ui),
            }
        });
    }
}


pub(crate) mod ui_prelude {
    pub use eframe::{Frame, egui};
    pub use egui::{Context, Ui};
}
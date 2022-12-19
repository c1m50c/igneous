use igneous_types::notebook::{Notebook, SelectionOption};
use super::ui_prelude::*;

mod editing;
mod select;
mod unlock;


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

    pub fn popup_error(ctx: &Context, frame: &mut Frame, ui: &mut Ui, error: &'static str) {

    }
}


impl eframe::App for IgneousApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                IgneousState::Select { .. } => select::select_state(&mut self.state, ctx, frame, ui),
                IgneousState::Unlock { .. } => unlock::unlock_state(&mut self.state, ctx, frame, ui),
                IgneousState::Editing { .. } => editing::editing_state(&mut self.state, ctx, frame, ui),
            }
        });
    }
}
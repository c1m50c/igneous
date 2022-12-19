use egui::Layout;
use igneous_types::notebook::{Notebook, SelectionOption};

use crate::ui_prelude::*;
use crate::IgneousState;


pub fn select_state(state: &mut IgneousState, ctx: &Context, frame: &mut Frame, ui: &mut Ui) {
    let IgneousState::Select { options } = state else { unreachable!() };
    let mut selected = None;

    ui.vertical_centered_justified(|ui| {
        ui.heading("Notebooks");

        let mut responses = Vec::with_capacity(options.len());

        ui.group(|ui| {
            options.iter_mut().for_each(|option| {
                let response = ui.button(option.identifier.as_str());
                responses.push((response, option.clone())); // TODO: No Clone
            });
        });

        ui.columns(2, |columns| {
            columns[0].button("Create");
            columns[1].button("Remove");
        });

        let maybe_clicked = responses.iter()
            .find(|(response, _)| response.clicked());

        if let Some((response, option)) = maybe_clicked {
            let Ok(notebook) = Notebook::from_path(&option.path) else {
                todo!("Add Error Popup")
            }; selected = Some(notebook);
        }
    });

    if let Some(notebook) = selected {
        *state = IgneousState::Editing { notebook };
    }
}
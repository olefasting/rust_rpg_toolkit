use std::ops::Deref;

use crate::gui::*;

pub struct Checkbox<'a> {
    id: Id,
    position: Option<Vec2>,
    label: String,
    value: &'a mut bool,
    allow_click_on_label: bool,
}

impl<'a> Checkbox<'a> {
    const ALLOW_CLICK_ON_LABEL: bool = true;

    pub fn new<P: Into<Option<Vec2>>>(id: Id, position: P, label: &str, value: &'a mut bool) -> Self {
        Checkbox {
            id,
            position: position.into(),
            label: label.to_string(),
            value,
            allow_click_on_label: Self::ALLOW_CLICK_ON_LABEL,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let gui_skins = storage::get::<GuiSkins>();

        if *self.value {
            ui.push_skin(&gui_skins.checkbox_selected);
        } else {
            ui.push_skin(&gui_skins.checkbox);
        }

        ui.separator();

        let label_size = ui.calc_size(&self.label);
        let checkbox_size = vec2(label_size.y - gui_skins.theme.separator_size, label_size.y - gui_skins.theme.separator_size);
        let total_size = vec2(checkbox_size.x + gui_skins.theme.separator_size, 0.0) + label_size;

        let mut group = widgets::Group::new(self.id, total_size);

        if let Some(position) = &self.position {
            group = group.position(position.clone());
        }

        group.ui(ui, |ui| {
            let checkbox = widgets::Button::new("")
                .size(checkbox_size)
                .ui(ui);

            if checkbox {
                *self.value = !*self.value;
            }

            ui.push_skin(&gui_skins.label_button);
            let label_btn = widgets::Button::new(self.label.deref())
                .position(vec2(checkbox_size.x + gui_skins.theme.separator_size, 0.0))
                .ui(ui);
            ui.pop_skin();

            if label_btn && self.allow_click_on_label {
                *self.value = !*self.value;
            }
        });

        ui.pop_skin();
    }
}
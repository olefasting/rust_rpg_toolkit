use crate::gui::*;
use macroquad::ui::Id;

pub fn draw_checkbox<P: Into<Option<Vec2>>>(ui: &mut Ui, id: Id, position: P, label: &str, value: &mut bool) {
    let gui_skins = storage::get::<GuiSkins>();

    if *value {
        ui.push_skin(&gui_skins.checkbox_selected);
    } else {
        ui.push_skin(&gui_skins.checkbox);
    }

    ui.separator();

    let size = (vec2(24.0, 0.0) ) + ui.calc_size(label);
    let mut group = widgets::Group::new(id, size);

    if let Some(position) = position.into() {
        group = group.position(position);
    }

    group.ui(ui, |ui| {
        let checkbox = widgets::Button::new("")
            .size(vec2(18.0, 18.0) )
            .ui(ui);

        if checkbox {
            *value = !*value;
        }

        ui.label(vec2(24.0, 0.0) , label);
    });

    ui.pop_skin();
}

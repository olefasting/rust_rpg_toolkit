use crate::gui::*;

pub fn draw_checkbox<P: Into<Option<Vec2>>>(ui: &mut Ui, position: P, label: &str, value: &mut bool) {
    let gui_skins = storage::get::<GuiSkins>();
    let scale = gui_skins.scale;

    if *value {
        ui.push_skin(&gui_skins.checkbox_selected);
    } else {
        ui.push_skin(&gui_skins.checkbox);
    }

    ui.separator();

    let size = (vec2(16.0, 0.0) * scale) + ui.calc_size(label);
    let mut group = widgets::Group::new(hash!(label, "checkbox"), size);

    if let Some(position) = position.into() {
        group = group.position(position);
    }

    group.ui(ui, |ui| {
        let checkbox = widgets::Button::new("")
            .size(vec2(12.0, 12.0) * scale)
            .position(vec2(0.0, 6.0) * scale)
            .ui(ui);

        if checkbox {
            *value = !*value;
        }

        //ui.same_line(16.0 * scale);

        ui.label(vec2(16.0, 0.0) * scale, label);
    });

    ui.pop_skin();
}

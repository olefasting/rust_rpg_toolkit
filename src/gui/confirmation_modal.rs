use crate::gui::*;

pub fn draw_confirmation_modal(ui: &mut Ui, body: Vec<String>) -> Option<bool> {
    let mut res = None;

    let mut size = vec2(0.0, GuiSkins::BUTTON_HEIGHT + GuiSkins::ELEMENT_MARGIN + GuiSkins::WINDOW_MARGIN_Y * 2.0);

    for line in &body {
        let line_size = ui.calc_size(line);

        if line_size.x + 50.0 > size.x {
            size.x = line_size.x + 50.0;
        }

        size.y += line_size.y;
    }

    let position = get_centered_on_screen(size);

    widgets::Window::new(hash!(), position, size)
        .titlebar(false)
        .ui(ui, |ui| {
            for line in &body {
                ui.label(None, line);
            }

            let buttons_y = size.y - GuiSkins::WINDOW_MARGIN_Y * 2.0 - GuiSkins::BUTTON_HEIGHT;
            let buttons_size = vec2((size.x - GuiSkins::ELEMENT_MARGIN) / 2.0 - GuiSkins::WINDOW_MARGIN_X, GuiSkins::BUTTON_HEIGHT);

            let confirm_btn = widgets::Button::new("Confirm")
                .position(vec2(0.0, buttons_y))
                .size(buttons_size)
                .ui(ui);

            if confirm_btn {
                res = Some(true);
            }

            let cancel_btn = widgets::Button::new("Cancel")
                .position(vec2(buttons_size.x + GuiSkins::ELEMENT_MARGIN, buttons_y))
                .size(buttons_size)
                .ui(ui);

            if cancel_btn {
                res = Some(false);
            }
        });

    res
}

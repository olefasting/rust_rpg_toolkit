use crate::gui::*;

pub async fn draw_chapter_select() -> (usize, String) {
    let gui_skins = storage::get::<GuiSkins>();
    let scenario = storage::get::<Scenario>();

    root_ui().push_skin(&gui_skins.default);
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        let scale = gui_skins.scale;

        let size = vec2(200.0 * scale, 300.0 * scale);
        let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);

        let mut result = None;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Chapter Select");

                ui.separator();

                for i in 0..scenario.chapters.len() {
                    let chapter = scenario.chapters.get(i).unwrap();
                    if ui.button(None, &chapter.title.clone()) {
                       result = Some((i, chapter.initial_map_id.clone()));
                    }
                }
            });

        if let Some(result) = result {
            root_ui().pop_skin();
            return result;
        }

        next_frame().await;
    }
}

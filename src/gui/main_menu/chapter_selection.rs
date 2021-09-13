use std::ops::Deref;

use crate::gui::*;

pub(crate) async fn draw_chapter_select_menu() -> Option<SceneTransitionParams> {
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        root_ui().push_skin(&gui_skins.default);

        let size = vec2(200.0, 250.0);
        let position = get_centered_on_screen(size);

        let mut result = None;
        let mut should_cancel = false;

        widgets::Window::new(hash!(), position, size)
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.push_skin(&gui_skins.header_label);
                ui.label(None, "Select Chapter");
                ui.pop_skin();

                ui.separator();

                let resources = storage::get::<Resources>();

                widgets::Group::new(hash!(), vec2(150.0, 144.0)).position(vec2(0.0, 27.0)).ui(ui, |ui| {
                    let len = resources.chapters.len();

                    let btn_width = if len > 4 {
                        140.0
                    } else {
                        150.0
                    };

                    for i in 0..len {
                        let chapter = resources.chapters.get(i).unwrap();

                        let chapter_btn = widgets::Button::new(chapter.title.deref())
                            .size(vec2(btn_width, 28.0))
                            .ui(ui);

                        if chapter_btn {
                            let params = SceneTransitionParams {
                                chapter_index: i,
                                map_id: chapter.initial_map_id.clone(),
                            };
                            result = Some(params);
                        }
                    }
                });

                let cancel_btn = widgets::Button::new("Cancel")
                    .position(vec2(0.0, 175.0))
                    .size(vec2(150.0, 28.0))
                    .ui(ui);

                if cancel_btn {
                    should_cancel = true;
                }
            });

        if result.is_some() || should_cancel {
            root_ui().pop_skin();

            if should_cancel {
                return None;
            } else {
                return result;
            }
        }

        next_frame().await;
    }
}

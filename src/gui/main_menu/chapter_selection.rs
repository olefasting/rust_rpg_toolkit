use std::ops::Deref;

use crate::gui::*;

pub(crate) async fn draw_chapter_select_menu() -> Option<SceneTransitionParams> {
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        root_ui().push_skin(&gui_skins.default);

        let mut result = None;
        let mut should_cancel = false;

        let resources = storage::get::<Resources>();

        let params = gui_skins.theme.menu_params.get("chapter_selection").cloned().unwrap();

        let mut index = 0;
        let params = MenuParams {
            options: resources.chapters.iter().map(|chapter| {
                let opt = MenuOption {
                    index,
                    title: chapter.title.clone(),
                    flags: Vec::new(),
                };
                index += 1;
                opt
            }).collect(),
            ..params
        };

        if let Some(i) = WindowBuilder::new_menu(&mut *root_ui(), hash!("chapter_selection"), &params) {
            let chapter = resources.chapters.get(i).unwrap();

            let transition = SceneTransitionParams {
                chapter_index: i,
                map_id: chapter.initial_map_id.clone(),
            };

            result = Some(transition);
        }

                // let cancel_btn = widgets::Button::new("Cancel")
                //     .position(vec2(0.0, 175.0))
                //     .size(vec2(150.0, 28.0))
                //     .ui(ui);
                //
                // if cancel_btn {
                //     should_cancel = true;
                // }

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

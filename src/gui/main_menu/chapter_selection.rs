use crate::gui::*;

pub(crate) async fn draw_chapter_select_menu() -> Option<SceneTransitionParams> {
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        root_ui().push_skin(&gui_skins.default);

        let mut result = None;
        let mut should_cancel = false;

        let resources = storage::get::<Resources>();

        let params = gui_skins.theme.menu_params.get("chapter_selection").cloned().unwrap();

        let mut index = 1; // 0 is cancel
        let mut options = params.options;
        for chapter in &resources.chapters {
            options.push(MenuOption {
                index,
                title: chapter.title.clone(),
                push_down: false,
            });
            index += 1;
        }

        let params = MenuParams {
            options,
            ..params
        };

        if let Some(i) = WindowBuilder::new_menu(&mut *root_ui(), hash!("chapter_selection"), &params) {
            if i == 0 {
                should_cancel = true;
            } else {
                let chapter = resources.chapters.get(i - 1).unwrap();

                let transition = SceneTransitionParams {
                    chapter_index: i,
                    map_id: chapter.initial_map_id.clone(),
                };

                result = Some(transition);
            }
        }

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

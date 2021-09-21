use crate::gui::*;

const OPT_CANCEL: usize = 0;

pub(crate) async fn draw_chapter_selection_menu() -> Option<(usize, String)> {
    loop {
        let gui_skins = storage::get::<GuiSkins>();
        root_ui().push_skin(&gui_skins.default);

        let mut result = None;
        let mut should_cancel = false;

        let resources = get_resources();

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

        if let Some(i) = MenuBuilder::new(hash!(), params).build(&mut *root_ui()) {
            if i == OPT_CANCEL {
                should_cancel = true;
            } else {
                let chapter_index = i - 1;
                let chapter = resources.chapters.get(chapter_index).unwrap();
                result = Some((chapter_index, chapter.initial_map_id.clone()));
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

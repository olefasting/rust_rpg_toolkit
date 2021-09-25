use crate::gui::*;
use crate::resources::CharacterClass;

const CLASS_SELECTION_MENU_ID: &'static str = "class_selection";

const WINDOW_WIDTH: f32 = 500.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub(crate) async fn draw_class_selection() -> Option<String> {
    let gui_skins = storage::get::<GuiSkins>();
    let resources = storage::get::<Resources>();
    let mut params = gui_skins.theme.menu_params.get(CLASS_SELECTION_MENU_ID).cloned().unwrap();

    let mut classes = Vec::new();

    for (_, class) in resources.character_classes.clone() {
        params.options.push(MenuOption {
            index: Some(classes.len()),
            title: Some(class.name.clone()),
            ..Default::default()
        });

        classes.push(class);
    }

    let builder = MenuBuilder::new(hash!(), params);

    let mut res = None;

    let mut should_cancel = false;

    loop {
        match builder.build(&mut *root_ui()) {
            MenuResult::Index(i) => {
                let class = classes.get(i).unwrap();
                res = Some(class.id.clone());
            }
            MenuResult::Cancel => {
                res = None;
                should_cancel = true;
            }
            _ => {}
        }

        if should_cancel || res.is_some() {
            return res;
        }

        next_frame().await;
    }
}
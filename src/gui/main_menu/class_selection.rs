use crate::gui::*;

const CLASS_SELECTION_MENU_ID: &str = "class_selection";

pub(crate) async fn draw_class_selection() -> Option<String> {
    let gui_skins = storage::get::<GuiSkins>();
    let resources = storage::get::<Resources>();
    let mut params = gui_skins
        .theme
        .menu_params
        .get(CLASS_SELECTION_MENU_ID)
        .cloned()
        .unwrap();

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

        end_frame().await;
    }
}

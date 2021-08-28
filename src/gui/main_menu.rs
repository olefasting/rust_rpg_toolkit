use crate::gui::*;

pub enum MainMenuSelection {
    NewGame { character: ExportedCharacter },
    LoadGame { save_game: SaveGame },
    Cancel,
    Quit,
}
//
// pub async fn draw_main_menu() -> MainMenuSelection {
//     let gui_skins = storage::get::<GuiSkins>();
//
//     root_ui().push_skin(&gui_skins.default);
//     loop {
//         let gui_skins = storage::get::<GuiSkins>();
//         let scale = gui_skins.scale;
//
//         let size = vec2(200.0 * scale, 300.0 * scale);
//         let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);
//
//         let mut selection = None;
//
//         widgets::Window::new(hash!(), position, size)
//             .titlebar(false)
//             .ui(&mut *root_ui(), |ui| {
//                 if ui.button(None, "New Game") {
//                     selection = draw_new_game_menu();
//                 }
//
//                 if ui.button(None, "Load Game") {
//                     selection = Some(MainMenuSelection::LoadGame);
//                 }
//
//                 if ui.button(None, "Quit") {
//                     selection = Some(MainMenuSelection::Quit);
//                 }
//             });
//
//         if let Some(selection) = selection {
//             root_ui().pop_skin();
//             return selection;
//         }
//
//         next_frame().await;
//     }
// }
//
// pub async fn draw_new_game_menu() -> MainMenuSelection {
//     let gui_skins = storage::get::<GuiSkins>();
//
//     root_ui().push_skin(&gui_skins.default);
//     loop {
//         let gui_skins = storage::get::<GuiSkins>();
//         let scale = gui_skins.scale;
//
//         let size = vec2(200.0 * scale, 300.0 * scale);
//         let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);
//
//         let mut selection = None;
//
//         widgets::Window::new(hash!(), position, size)
//             .titlebar(false)
//             .ui(&mut *root_ui(), |ui| {
//                 ui.label(None, "New Game");
//
//                 ui.separator();
//
//                 if ui.button(None, "Create Character") {
//                     selection = draw_create_character_menu();
//                 }
//
//                 if ui.button(None, "Import Character") {
//                     selection = draw_import_character_menu();
//                 }
//
//                 if ui.button(None, "Cancel") {
//                     selection = draw_import_character_menu();
//                 }
//             });
//
//         if let Some(selection) = selection {
//             root_ui().pop_skin();
//             return selection;
//         }
//
//         next_frame().await;
//     }
// }
//
// pub async fn draw_create_character_menu() -> MainMenuSelection {
//     let gui_skins = storage::get::<GuiSkins>();
//
//     root_ui().push_skin(&gui_skins.default);
//     loop {
//         let gui_skins = storage::get::<GuiSkins>();
//         let scale = gui_skins.scale;
//
//         let size = vec2(200.0 * scale, 300.0 * scale);
//         let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);
//
//         let mut selection = None;
//
//         widgets::Window::new(hash!(), position, size)
//             .titlebar(false)
//             .ui(&mut *root_ui(), |ui| {
//                 ui.label(None, "New Game");
//
//                 ui.separator();
//
//                 if ui.button(None, "Create Character") {
//                     selection = draw_create_character_menu();
//                 }
//
//                 if ui.button(None, "Import Character") {
//                     selection = draw_import_character_menu();
//                 }
//
//                 if ui.button(None, "Cancel") {
//                     selection = draw_import_character_menu();
//                 }
//             });
//
//         if let Some(selection) = selection {
//             root_ui().pop_skin();
//             return selection;
//         }
//
//         next_frame().await;
//     }
// }
//
//
// pub async fn draw_import_character_menu() -> MainMenuSelection {
//     let gui_skins = storage::get::<GuiSkins>();
//
//     root_ui().push_skin(&gui_skins.default);
//     loop {
//         let gui_skins = storage::get::<GuiSkins>();
//         let scale = gui_skins.scale;
//
//         let size = vec2(200.0 * scale, 300.0 * scale);
//         let position = vec2((screen_width() - size.x)  / 2.0, (screen_height() - size.y) / 2.0);
//
//         let mut selection = None;
//
//         widgets::Window::new(hash!(), position, size)
//             .titlebar(false)
//             .ui(&mut *root_ui(), |ui| {
//                 ui.label(None, "Select Character");
//
//                 ui.separator();
//
//                 if ui.button(None, "Load") {
//                     selection = draw_create_character_menu();
//                 }
//
//                 if ui.button(None, "Cancel") {
//                     selection = draw_import_character_menu();
//                 }
//             });
//
//         if let Some(selection) = selection {
//             root_ui().pop_skin();
//             return selection;
//         }
//
//         next_frame().await;
//     }
// }

pub use macroquad::{
    ui::{
        Drag,
        hash,
        root_ui,
        Skin,
        Ui,
        widgets,
        Id,
    },
};

pub use character::draw_character_window;
pub use confirmation_modal::draw_confirmation_modal;
pub use dialogue::draw_dialogue_window;
pub use game_menu::draw_game_menu;
pub use inventory::draw_inventory_window;
pub use main_menu::{
    draw_main_menu,
    MainMenuResult,
};
pub use theme::{GuiSkins, GuiTheme};
pub use checkbox::Checkbox;

use crate::prelude::*;

mod inventory;
mod character;
mod dialogue;
mod game_menu;
mod theme;
mod main_menu;
mod confirmation_modal;
mod checkbox;

pub async fn load_gui_theme(game_params: &GameParams) -> Result<(), FileError> {
    let path = format!("{}/gui_theme.json", &game_params.data_path);
    let bytes = load_file(&path).await?;
    let gui_theme = serde_json::from_slice(&bytes)
        .expect(&format!("Error when parsing gui theme '{}'", path));
    let gui_skins = GuiSkins::new(gui_theme);
    storage::store(gui_skins);
    Ok(())
}

pub fn draw_gui() {
    if let Some(mut game_state) = scene::find_node_by_type::<GameState>() {
        if let Some(mut player) = Actor::find_by_player_id(&game_state.local_player_id) {
            if game_state.should_show_character_window {
                draw_character_window(&*player);
            }
            if game_state.should_show_inventory_window {
                draw_inventory_window(&mut *player);
            }
            draw_dialogue_window(&mut *player);
            if game_state.should_show_game_menu {
                draw_game_menu(&mut game_state);
            }
        }
    }
}

pub fn get_centered(size: Vec2, bounds: Vec2) -> Vec2 {
    (bounds - size) / 2.0
}

pub fn get_centered_on_screen(size: Vec2) -> Vec2 {
    let bounds = vec2(screen_width(), screen_height());
    get_centered(size, bounds)
}
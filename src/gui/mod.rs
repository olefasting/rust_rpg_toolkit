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
pub use theme::{GuiSkins, GuiTheme, MenuPosition, MenuParams, MenuOption};
pub use checkbox::Checkbox;

use crate::prelude::*;
use std::ops::Deref;
use crate::gui::theme::MENU_OPTION_FLAG_FIX_TO_BOTTOM;

mod inventory;
mod character;
mod dialogue;
mod game_menu;
mod theme;
mod main_menu;
mod confirmation_modal;
mod checkbox;

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

pub struct WindowBuilder {
    id: Id,
    size: Vec2,
    position: Vec2,
    title: Option<String>,
    is_static: bool,
}

impl WindowBuilder {
    pub fn new(id: Id, size: Vec2) -> Self {
        WindowBuilder {
            id,
            size,
            position: Vec2::ZERO,
            title: None,
            is_static: false,
        }
    }

    pub fn with_title(self, title: &str) -> Self {
        WindowBuilder {
            id: self.id,
            size: self.size,
            position: self.position,
            title: Some(title.to_string()),
            is_static: self.is_static,
        }
    }

    pub fn with_pos(self, position: Vec2, is_static: bool) -> Self {
        WindowBuilder {
            id: self.id,
            size: self.size,
            position,
            title: self.title,
            is_static,
        }
    }

    pub fn with_centered_pos(self, is_static: bool) -> Self {
        WindowBuilder {
            id: self.id,
            size: self.size,
            position: get_centered_on_screen(self.size),
            title: self.title,
            is_static,
        }
    }

    pub fn build<F: FnOnce(&mut Ui)>(self, ui: &mut Ui, f: F) -> bool {
        let window = widgets::Window::new(self.id, self.position, self.size)
            .movable(self.is_static)
            .titlebar(false);

        if let Some(title) = self.title {
            return window.ui(ui, |ui| {
                let gui_skins = storage::get::<GuiSkins>();
                ui.push_skin(&gui_skins.window_title);
                ui.label(None, &title);
                ui.pop_skin();

                f(ui);
            })
        }

        window.ui(ui, f)
    }

    pub fn new_menu(ui: &mut Ui, id: Id, params: &MenuParams) -> Option<usize> {
        let mut res = None;

        let mut builder = Self::new(id, params.size);

        builder = match params.position {
            MenuPosition::Normal(position) => builder.with_pos(position, params.is_static),
            MenuPosition::Centered => builder.with_centered_pos(params.is_static),
            MenuPosition::CenteredHorizontally(y) => builder.with_pos(vec2(get_centered_on_screen(params.size).x, y), params.is_static),
            MenuPosition::CenteredVertically(x) => builder.with_pos(vec2(x, get_centered_on_screen(params.size).y), params.is_static),
        };

        builder.title = params.title.clone();

        builder.build(ui, |ui| {
            let gui_skins = storage::get::<GuiSkins>();

            let mut bottom_stack_top = params.size.y - gui_skins.theme.window_margins.top - gui_skins.theme.window_margins.bottom - gui_skins.theme.button_height;

            for opt in &params.options {
                let size = vec2(
                    params.size.x - gui_skins.theme.window_margins.left - gui_skins.theme.window_margins.right,
                    gui_skins.theme.button_height,
                );

                let mut btn = widgets::Button::new(opt.title.deref());

                for flag in &opt.flags {
                    if flag == MENU_OPTION_FLAG_FIX_TO_BOTTOM {
                        btn = btn.position(vec2(0.0, bottom_stack_top));
                        bottom_stack_top -= gui_skins.theme.button_height;
                    }
                }

                btn = btn.size(size);

                if btn.ui(ui) {
                    res = Some(opt.index);
                }
            }
        });
        res
    }
}
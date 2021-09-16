use crate::gui::*;

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
        let mut builder = Self::new(id, params.size);

        builder = match params.position {
            MenuPosition::Normal(position) => builder.with_pos(position, params.is_static),
            MenuPosition::Centered => builder.with_centered_pos(params.is_static),
            MenuPosition::CenteredHorizontally(y) => builder.with_pos(vec2(get_centered_on_screen(params.size).x, y), params.is_static),
            MenuPosition::CenteredVertically(x) => builder.with_pos(vec2(x, get_centered_on_screen(params.size).y), params.is_static),
        };

        builder.title = params.title.clone();

        let mut res = None;

        builder.build(ui, |ui| {
            let gui_skins = storage::get::<GuiSkins>();

            let mut next_top_y = 0.0;
            let mut next_bottom_y = params.size.y - gui_skins.theme.window_margins.top - gui_skins.theme.window_margins.bottom - gui_skins.theme.button_height;

            if let Some(title) = &params.title {
                ui.push_skin(&gui_skins.window_title);
                next_top_y += ui.calc_size(title).y;
                ui.pop_skin();
            }

            for opt in &params.options {
                let mut btn = widgets::Button::new(opt.title.deref());

                let mut x_position = 0.0;

                match params.button_style {
                    MenuButtonStyle::FullWidth => {
                        let size = vec2(params.size.x - gui_skins.theme.window_margins.left - gui_skins.theme.window_margins.right,
                                        gui_skins.theme.button_height);
                        btn = btn.size(size);
                    }
                    MenuButtonStyle::Centered => {
                        let label_size = ui.calc_size(&opt.title);
                        let button_width = label_size.x + gui_skins.theme.button_margins.left + gui_skins.theme.button_margins.right;
                        let container_width = params.size.x - gui_skins.theme.window_margins.left - gui_skins.theme.window_margins.right;
                        x_position = container_width / 2.0 - button_width / 2.0;
                    }
                    MenuButtonStyle::None => {}
                };

                if opt.push_down {
                    btn = btn.position(vec2(x_position, next_bottom_y));
                    next_bottom_y -= gui_skins.theme.button_height;
                } else {
                    if params.button_style == MenuButtonStyle::Centered {
                        btn = btn.position(vec2(x_position, next_top_y));
                    }
                    next_top_y += gui_skins.theme.button_height;
                }

                if btn.ui(ui) {
                    res = Some(opt.index);
                }
            }
        });

        res
    }
}
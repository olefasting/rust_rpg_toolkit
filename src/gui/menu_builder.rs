use crate::gui::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MenuResult {
    Cancel,
    Index(usize),
    None,
}

impl MenuResult {
    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    pub fn is_cancel(&self) -> bool {
        *self == Self::Cancel
    }

    pub fn is_index(&self) -> bool {
        if let Self::Index(_) = *self {
            return true;
        }
        false
    }
}

pub struct MenuBuilder {
    params: MenuParams,
    window_builder: WindowBuilder,
}

impl MenuBuilder {
    pub fn new(id: Id, params: MenuParams) -> Self {
        let mut window_builder = WindowBuilder::new(id, params.size);

        if let Some(title) = &params.title {
            window_builder = window_builder.with_title(title);
        }

        window_builder = match params.position {
            MenuPosition::Normal(position) => window_builder.with_pos(position, params.is_static),
            MenuPosition::Centered => window_builder.with_centered_pos(params.is_static),
            MenuPosition::CenteredHorizontally(y) => window_builder.with_pos(vec2(get_centered_on_screen(params.size).x, y), params.is_static),
            MenuPosition::CenteredVertically(x) => window_builder.with_pos(vec2(x, get_centered_on_screen(params.size).y), params.is_static),
        };

        MenuBuilder {
            params,
            window_builder,
        }
    }

    pub fn build(&self, ui: &mut Ui) -> MenuResult {
        let mut res = MenuResult::None;

        self.window_builder.build(ui, |ui| {
            let gui_skins = storage::get::<GuiSkins>();

            let mut next_top_y = 0.0;

            if let Some(title) = &self.params.title {
                ui.push_skin(&gui_skins.window_title);
                next_top_y += ui.calc_size(title).y;
                ui.pop_skin();
            }

            let mut button_height = gui_skins.theme.button_height;

            if self.params.button_style.is_label() {
                ui.push_skin(&gui_skins.label_button);
                button_height = ui.calc_size("Label").y;
            }

            let mut opts = self.params.options.clone();
            opts.sort_by(|a, b| a.index.cmp(&b.index));

            let mut next_bottom_y = self.params.size.y - gui_skins.theme.window_margins.top - gui_skins.theme.window_margins.bottom;

            let mut has_cancel = false;
            let mut indices = Vec::new();
            for opt in &mut opts {
                if let Some(i) = opt.index {
                    assert_eq!(indices.contains(&i), false, "Duplicate opt indices '{}' in menu '{}'", i, self.params.id);
                    indices.push(i);
                } else if opt.is_cancel {
                    assert_eq!(has_cancel, false, "Multiple cancel options in menu '{}'!", self.params.id);
                    has_cancel = true;

                    if opt.title.is_none() {
                        opt.title = Some("Cancel".to_string());
                    }
                } else {
                    panic!("Encountered a menu option without an index in menu '{}'!", self.params.id);
                }

                if opt.push_down {
                    next_bottom_y -= button_height + 2.0;
                }
            }

            for opt in &opts {
                let mut style = opt.style_override.unwrap_or(self.params.button_style);

                let mut title = None;
                if style != MenuButtonStyle::Label {
                    title = opt.title.clone();
                }

                let title = title.unwrap_or("".to_string());

                let mut btn = widgets::Button::new(title.as_ref());

                let mut x_position = 0.0;

                match style {
                    MenuButtonStyle::FullWidth | MenuButtonStyle::Label | MenuButtonStyle::CenteredLabel => {
                        let size = vec2(self.params.size.x - gui_skins.theme.window_margins.left - gui_skins.theme.window_margins.right, gui_skins.theme.button_height);
                        btn = btn.size(size);
                    }
                    MenuButtonStyle::Centered => {
                        let label_size = ui.calc_size(&title);
                        let button_width = label_size.x + gui_skins.theme.button_margins.left + gui_skins.theme.button_margins.right;
                        let container_width = self.params.size.x - gui_skins.theme.window_margins.left - gui_skins.theme.window_margins.right;
                        x_position = container_width / 2.0 - button_width / 2.0;
                    }
                    MenuButtonStyle::None => {}
                };

                let mut button_height = button_height;

                if style != self.params.button_style {
                    if style.is_label() {
                        button_height = ui.calc_size(&title).y;
                    } else {
                        button_height = gui_skins.theme.button_height;
                    }
                }

                let position = if opt.push_down {
                    let position = vec2(x_position, next_bottom_y);
                    next_bottom_y += button_height + 2.0;
                    position
                } else {
                    let position = vec2(x_position, next_top_y);
                    next_top_y += button_height;
                    position
                };

                if style == MenuButtonStyle::Label {
                    if let Some(title) = opt.title.clone() {
                        ui.button(position, title.deref());
                    }
                }

                btn = btn.position(position);
                if btn.ui(ui) {
                    if opt.is_cancel {
                        res = MenuResult::Cancel;
                    } else {
                        res = MenuResult::Index(opt.index.unwrap());
                    }
                }
            }

            if self.params.button_style.is_label() {
                ui.pop_skin();
            }
        });

        res
    }
}
use crate::gui::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MenuButtonStyle {
    Normal {
        #[serde(default)]
        is_condensed: bool,
    },
    LabelOnly {
        #[serde(default)]
        is_centered: bool,
    },
    Custom {
        builder_id: String,
    },
}

impl MenuButtonStyle {
    pub fn is_normal(&self) -> bool {
        if let Self::Normal { is_condensed: _ } = self {
            return true;
        }

        false
    }

    pub fn is_label_only(&self) -> bool {
        if let Self::LabelOnly { is_centered: _ } = self {
            return true;
        }

        false
    }

    pub fn is_custom(&self) -> bool {
        if let Self::Custom { builder_id: _ } = self {
            return true;
        }

        false
    }
}

impl Default for MenuButtonStyle {
    fn default() -> Self {
        Self::Normal {
            is_condensed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MenuPosition {
    Normal(#[serde(with = "json::def_vec2")] Vec2),
    CenteredVertically(f32),
    CenteredHorizontally(f32),
    Centered,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MenuOption {
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub is_pushed_down: bool,
    #[serde(default)]
    pub style_override: Option<MenuButtonStyle>,
    #[serde(default, skip_serializing_if = "helpers::is_false")]
    pub is_cancel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuParams {
    pub id: String,
    #[serde(with = "json::def_vec2")]
    pub size: Vec2,
    pub position: MenuPosition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<MenuOption>,
    pub button_style: MenuButtonStyle,
    #[serde(default)]
    pub is_static: bool,
}

impl Default for MenuParams {
    fn default() -> Self {
        MenuParams {
            id: "".to_string(),
            size: Vec2::ZERO,
            position: MenuPosition::Centered,
            title: None,
            options: Vec::new(),
            button_style: Default::default(),
            is_static: true,
        }
    }
}

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
    id: Id,
    params: MenuParams,
}

impl MenuBuilder {
    pub fn new(id: Id, params: MenuParams) -> Self {
        MenuBuilder { id, params }
    }

    pub fn build(&self, ui: &mut Ui) -> MenuResult {
        let mut window_builder = WindowBuilder::new(self.id, self.params.size);

        if let Some(title) = &self.params.title {
            window_builder = window_builder.with_title(title);
        }

        window_builder = match self.params.position {
            MenuPosition::Normal(position) => {
                window_builder.with_pos(position, self.params.is_static)
            }
            MenuPosition::Centered => window_builder.with_centered_pos(self.params.is_static),
            MenuPosition::CenteredHorizontally(y) => window_builder.with_pos(
                vec2(get_centered_on_screen(self.params.size).x, y),
                self.params.is_static,
            ),
            MenuPosition::CenteredVertically(x) => window_builder.with_pos(
                vec2(x, get_centered_on_screen(self.params.size).y),
                self.params.is_static,
            ),
        };

        let mut res = MenuResult::None;

        window_builder.build(ui, |ui| {
            let gui_skins = storage::get::<GuiSkins>();
            ui.push_skin(&gui_skins.default);

            let full_width = self.params.size.x
                - (gui_skins.theme.window_margins.left + gui_skins.theme.window_margins.right);

            let mut opts = self.params.options.clone();
            opts.sort_by(|a, b| a.index.cmp(&b.index));

            let mut has_cancel = false;
            let mut indices = Vec::new();
            for opt in &mut opts {
                if let Some(i) = opt.index {
                    assert!(
                        !indices.contains(&i),
                        "Duplicate opt indices '{}' in menu '{}'",
                        i,
                        self.params.id
                    );
                    indices.push(i);
                } else if opt.is_cancel {
                    assert!(
                        !has_cancel,
                        "Multiple cancel options in menu '{}'!",
                        self.params.id
                    );
                    has_cancel = true;
                } else {
                    panic!(
                        "Encountered a menu option without an index in menu '{}'!",
                        self.params.id
                    );
                }
            }

            let mut builders = Vec::new();

            for opt in &opts {
                let style = opt
                    .style_override
                    .clone()
                    .unwrap_or_else(|| self.params.button_style.clone());
                let mut builder = if let MenuButtonStyle::Custom { builder_id } = &style {
                    get_button_builder(builder_id)
                } else {
                    let mut builder = ButtonBuilder::new();

                    match style {
                        MenuButtonStyle::Normal { is_condensed } => {
                            let width = Some(full_width);
                            let style = ButtonStyle::Normal {
                                width,
                                is_condensed,
                            };
                            builder = builder.with_style(style);
                        }
                        MenuButtonStyle::LabelOnly { is_centered } => {
                            let width = full_width;
                            let style = ButtonStyle::LabelOnly { width, is_centered };
                            builder = builder.with_style(style);
                        }
                        _ => {}
                    };

                    builder
                };

                if let Some(title) = &opt.title {
                    builder = builder.with_label(title);
                }

                let menu_result = if opt.is_cancel {
                    MenuResult::Cancel
                } else {
                    MenuResult::Index(opt.index.unwrap())
                };

                let params = (builder, opt.is_pushed_down, menu_result);

                builders.push(params);
            }

            let mut next_top_y = 0.0;
            if let Some(title) = &self.params.title {
                ui.push_skin(&gui_skins.window_title);
                next_top_y += ui.calc_size(title).y + 2.0;
                ui.pop_skin();
            }

            let mut next_bottom_y = self.params.size.y
                - gui_skins.theme.window_margins.top
                - gui_skins.theme.window_margins.bottom;

            for (builder, is_push_down, _) in &builders {
                if *is_push_down {
                    next_bottom_y -= builder.get_height() + 2.0;
                }
            }

            for (builder, is_push_down, menu_result) in builders {
                let mut position = Vec2::ZERO;

                let button_height = builder.get_height();

                if is_push_down {
                    position.y = next_bottom_y;
                    next_bottom_y += button_height + 2.0;
                } else {
                    position.y = next_top_y;
                    next_top_y += button_height + 2.0;
                }

                if builder.with_position(position).build(ui) {
                    res = menu_result;
                }
            }

            ui.pop_skin();
        });

        res
    }
}

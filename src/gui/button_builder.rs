use crate::gui::*;

pub type ButtonBuildFunc = fn(ui: &mut Ui, size: Vec2, position: Option<Vec2>, label: &Option<String>, is_inactive: bool) -> bool;

#[derive(Copy, Clone)]
pub enum ButtonStyle {
    Normal { width: Option<f32>, is_condensed: bool },
    LabelOnly { width: f32, is_centered: bool },
    Custom { size: Vec2, should_handle_clicks: bool, build_func: ButtonBuildFunc },
}

impl ButtonStyle {
    pub fn is_normal(&self) -> bool {
        if let Self::Normal { width: _, is_condensed: _ } = self {
            return true;
        }

        false
    }

    pub fn is_label_only(&self) -> bool {
        if let Self::LabelOnly { width: _, is_centered: _ } = self {
            return true;
        }

        false
    }

    pub fn is_custom(&self) -> bool {
        if let Self::Custom { size: _, should_handle_clicks: _, build_func: _ } = self {
            return true;
        }

        false
    }
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Normal { width: None, is_condensed: false }
    }
}

static mut BUTTON_BUILDERS: Option<HashMap<String, ButtonBuilder>> = None;

unsafe fn get_directory() -> &'static mut HashMap<String, ButtonBuilder> {
    if BUTTON_BUILDERS.is_none() {
        let map = HashMap::new();
        BUTTON_BUILDERS = Some(map);
    }

    BUTTON_BUILDERS.as_mut().unwrap()
}

pub fn register_button_builder(id: &str, builder: ButtonBuilder) {
    unsafe {
        get_directory().insert(id.to_string(), builder);
    }
}

pub fn try_get_button_builder(id: &str) -> Option<ButtonBuilder> {
    unsafe {
        get_directory().get(id).cloned()
    }
}

pub fn get_button_builder(id: &str) -> ButtonBuilder {
    try_get_button_builder(id).unwrap()
}

#[derive(Clone)]
pub struct ButtonBuilder {
    position: Option<Vec2>,
    label: Option<String>,
    style: ButtonStyle,
    is_highlighted: bool,
    is_inactive: bool,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            position: None,
            label: None,
            style: Default::default(),
            is_highlighted: false,
            is_inactive: false,
        }
    }

    pub fn with_position(self, position: Vec2) -> Self {
        ButtonBuilder {
            position: Some(position),
            ..self
        }
    }

    pub fn with_label(self, label: &str) -> Self {
        ButtonBuilder {
            label: Some(label.to_string()),
            ..self
        }
    }

    pub fn with_style(self, style: ButtonStyle) -> Self {
        ButtonBuilder {
            style,
            ..self
        }
    }

    pub fn set_highlighted(self) -> Self {
        ButtonBuilder {
            is_highlighted: true,
            ..self
        }
    }

    pub fn set_inactive(self) -> Self {
        ButtonBuilder {
            is_inactive: true,
            ..self
        }
    }

    pub fn add_to_directory(self, id: &str) {
        register_button_builder(id, self);
    }

    pub fn build(&self, ui: &mut Ui) -> bool {
        let gui_skins = storage::get::<GuiSkins>();

        if let ButtonStyle::Custom { size, should_handle_clicks, build_func } = self.style {
            let mut res = false;

            ui.push_skin(&gui_skins.custom_button);
            widgets::Group::new(hash!(), size).ui(ui, |ui| {
                ui.pop_skin();

                ui.push_skin(&gui_skins.default);
                res = build_func(ui, size, self.position, &self.label, self.is_inactive);
                ui.pop_skin();

                if should_handle_clicks {
                    ui.push_skin(&gui_skins.custom_button);
                    res = res || widgets::Button::new("").size(size).ui(ui);
                    ui.pop_skin();
                }
            });

            res
        } else {
            let label = self.label.clone().unwrap_or("".to_string());

            let mut btn = widgets::Button::new(label.deref());

            let height = self.get_height();

            let position = self.position.unwrap_or(Vec2::ZERO);

            match self.style {
                ButtonStyle::Normal { width, is_condensed } => {
                    if is_condensed {
                        if self.is_inactive {
                            ui.push_skin(&gui_skins.condensed_button_inactive);
                        } else {
                            ui.push_skin(&gui_skins.condensed_button);
                        }
                    } else {
                        if self.is_inactive {
                            ui.push_skin(&gui_skins.inactive_button);
                        } else {
                            ui.push_skin(&gui_skins.default);
                        }
                    }

                    let width = if let Some(width) = width {
                        width
                    } else {
                        let mut label_width = 0.0;
                        if let Some(label) = &self.label {
                            label_width += ui.calc_size(label).x;
                        }

                        label_width + gui_skins.theme.button_margins.left + gui_skins.theme.button_margins.right
                    };

                    let size = vec2(width, height);
                    btn = btn.size(size);
                }
                ButtonStyle::LabelOnly { width, is_centered } => {
                    if self.is_inactive {
                        ui.push_skin(&gui_skins.label_button_inactive);
                    } else if self.is_highlighted {
                        ui.push_skin(&gui_skins.label_button_highlighted);
                    } else {
                        ui.push_skin(&gui_skins.label_button);
                    }

                    let size = vec2(width, height);
                    if is_centered {
                        btn = btn.size(size);
                    } else {
                        btn.position(position).ui(ui);

                        btn = widgets::Button::new("").size(size);
                    }
                }
                _ => {}
            };

            let res = btn.position(position).ui(ui);

            ui.pop_skin();

            res
        }
    }

    pub fn get_height(&self) -> f32 {
        let gui_skins = storage::get::<GuiSkins>();
        match self.style {
            ButtonStyle::Custom { size, should_handle_clicks: _, build_func: _ } => size.y,
            _ => gui_skins.theme.button_height,
        }
    }
}
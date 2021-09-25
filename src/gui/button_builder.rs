use crate::gui::*;

#[derive(Copy, Clone)]
pub enum ButtonStyle {
    None,
    SetWidth { width: f32 },
    LabelOnly { width: f32, is_centered: bool },
    Custom { size: Vec2, should_handle_clicks: bool, build_func: fn(ui: &mut Ui) -> bool },
}

impl ButtonStyle {
    pub fn is_none(&self) -> bool {
        if let Self::None = self {
            return true;
        }

        false
    }

    pub fn is_set_width(&self) -> bool {
        if let Self::SetWidth { width: _ } = self {
            return true;
        }

        false
    }

    pub fn is_label(&self) -> bool {
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
        Self::None
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
    is_inactive: bool,
}

impl ButtonBuilder {
    pub fn new() -> Self {
        ButtonBuilder {
            position: None,
            label: None,
            style: Default::default(),
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

    pub fn is_inactive(self) -> Self {
        ButtonBuilder {
            is_inactive: true,
            ..self
        }
    }

    pub fn add_to_directory(self, id: &str) {
        register_button_builder(id, self);
    }

    pub fn build(&self, ui: &mut Ui) -> bool {
        if let ButtonStyle::Custom { size, should_handle_clicks, build_func } = self.style {
            let mut res = false;

            let gui_skins = storage::get::<GuiSkins>();
            ui.push_skin(&gui_skins.custom_button);
            widgets::Group::new(hash!(), size).ui(ui, |ui| {
                ui.pop_skin();

                res = build_func(ui);

                if should_handle_clicks {
                    ui.push_skin(&gui_skins.custom_button);
                    res = res || widgets::Button::new("").size(size).ui(ui);
                    ui.pop_skin();
                }
            });

            res
        } else {
            let gui_skins = storage::get::<GuiSkins>();
            if self.style.is_label() {
                let skin = if self.is_inactive {
                    &gui_skins.label_button_inactive
                } else {
                    &gui_skins.label_button
                };

                ui.push_skin(skin);
            }

            let label = self.label.clone().unwrap_or("".to_string());

            let mut btn = widgets::Button::new(label.deref());

            let height = self.get_height();

            match self.style {
                ButtonStyle::SetWidth { width } => {
                    let size = vec2(width, height);
                    btn = btn.size(size);
                }
                ButtonStyle::LabelOnly { width, is_centered } => {
                    let size = vec2(width, height);
                    if is_centered {
                        btn = btn.size(size);
                    } else {
                        let position = self.position.expect("Label buttons, without centered text, need an absolute position!");
                        btn.position(position).ui(ui);

                        btn = widgets::Button::new("").size(size);
                    }
                }
                _ => {}
            };

            if let Some(position) = self.position {
                btn = btn.position(position)
            }

            let res = btn.ui(ui);

            if self.style.is_label() {
                ui.pop_skin();
            }

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
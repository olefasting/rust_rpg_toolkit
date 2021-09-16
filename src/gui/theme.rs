use crate::{
    gui::*,
};

fn sub_offsets(a: RectOffset, b: RectOffset) -> RectOffset {
    RectOffset::new(a.left - b.left, a.right - b.right, a.top - b.top, a.bottom - b.bottom)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiImage {
    pub image_id: String,
    #[serde(with = "json::RectOffsetDef")]
    pub margins: RectOffset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiTheme {
    pub font_size: u16,
    pub header_font_size: u16,
    pub window_title_size: u16,
    pub button_font_size: u16,
    #[serde(with = "json::ColorDef")]
    pub text_color: Color,
    #[serde(with = "json::ColorDef")]
    pub highlight_text_color: Color,
    #[serde(with = "json::ColorDef")]
    pub warning_text_color: Color,
    #[serde(with = "json::ColorDef")]
    pub window_title_color: Color,
    #[serde(with = "json::ColorDef")]
    pub editbox_text_color: Color,
    #[serde(with = "json::ColorDef")]
    pub button_text_color: Color,
    #[serde(with = "json::ColorDef")]
    pub button_text_color_inactive: Color,
    #[serde(with = "json::RectOffsetDef")]
    pub window_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub label_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub header_label_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub window_title_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub button_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub editbox_margins: RectOffset,
    #[serde(with = "json::RectOffsetDef")]
    pub checkbox_margins: RectOffset,
    pub separator_size: f32,
    pub button_height: f32,
    #[serde(with = "json::ColorDef")]
    pub group_border_color: Color,
    #[serde(with = "json::ColorDef")]
    pub group_border_color_hovered: Color,
    #[serde(with = "json::ColorDef")]
    pub group_border_color_clicked: Color,
    #[serde(with = "json::RectOffsetDef")]
    pub group_margins: RectOffset,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_color: Color,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_color_hovered: Color,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_color_clicked: Color,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_handle_color: Color,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_handle_color_hovered: Color,
    #[serde(with = "json::ColorDef")]
    pub scrollbar_handle_color_clicked: Color,
    pub window_bg: GuiImage,
    pub button_bg: GuiImage,
    pub button_bg_hovered: GuiImage,
    pub button_bg_clicked: GuiImage,
    pub button_bg_inactive: GuiImage,
    pub editbox_bg: GuiImage,
    pub checkbox_bg: GuiImage,
    pub checkbox_bg_hovered: GuiImage,
    pub checkbox_bg_clicked: GuiImage,
    pub checkbox_bg_selected: GuiImage,
    pub checkbox_bg_selected_hovered: GuiImage,
}

impl Default for GuiTheme {
    fn default() -> Self {
        let button_bg_margins = RectOffset::new(8.0, 8.0, 4.0, 4.0);
        let checkbox_bg_margins = RectOffset::new(2.0, 2.0, 2.0, 2.0);

        GuiTheme {
            font_size: 16,
            header_font_size: 18,
            window_title_size: 18,
            button_font_size: 16,
            text_color: Color::from_rgba(200, 200, 160, 255),
            highlight_text_color: Color::from_rgba(200, 200, 160, 255),
            window_title_color: Color::from_rgba(200, 200, 160, 255),
            warning_text_color: color::RED,
            editbox_text_color: Color::from_rgba(200, 200, 160, 255),
            button_text_color: Color::from_rgba(200, 200, 160, 255),
            button_text_color_inactive: Color::from_rgba(200, 200, 160, 255),
            window_margins: RectOffset::new(25.0, 25.0, 25.0, 25.0),
            label_margins: RectOffset::new(0.0, 0.0, 4.0, 4.0),
            header_label_margins: RectOffset::new(0.0, 0.0, 4.0, 8.0),
            window_title_margins: RectOffset::new(0.0, 0.0, 4.0, 8.0),
            button_margins: RectOffset::new(16.0, 16.0, 6.0, 6.0),
            editbox_margins: RectOffset::new(14.0, 14.0, 4.0, 0.0),
            checkbox_margins: RectOffset::new(0.0, 0.0, 4.0, 4.0),
            separator_size: 8.0,
            button_height: 32.0,
            group_border_color: Color::from_rgba(0, 0, 0, 0),
            group_border_color_hovered: Color::from_rgba(0, 0, 0, 0),
            group_border_color_clicked: Color::from_rgba(0, 0, 0, 0),
            group_margins: RectOffset::new(0.0, 0.0, 0.0, 0.0),
            scrollbar_color: Color::from_rgba(58, 68,68, 255),
            scrollbar_color_hovered: Color::from_rgba(58, 68,102, 255),
            scrollbar_color_clicked: Color::from_rgba(58, 68,102, 255),
            scrollbar_handle_color: Color::from_rgba(58, 68,68, 255),
            scrollbar_handle_color_hovered: Color::from_rgba(58, 68,102, 255),
            scrollbar_handle_color_clicked: Color::from_rgba(58, 68,102, 255),
            window_bg: GuiImage {
                image_id: "panel_01".to_string(),
                margins: RectOffset::new(52.0, 52.0, 52.0, 52.0),
            },
            button_bg: GuiImage {
                image_id: "btn_01".to_string(),
                margins: button_bg_margins,
            },
            button_bg_hovered: GuiImage {
                image_id: "btn_01_hovered".to_string(),
                margins: button_bg_margins,
            },
            button_bg_clicked: GuiImage {
                image_id: "btn_01_clicked".to_string(),
                margins: button_bg_margins,
            },
            button_bg_inactive: GuiImage {
                image_id: "btn_01_inactive".to_string(),
                margins: button_bg_margins,
            },
            editbox_bg: GuiImage {
                image_id: "editbox_01".to_string(),
                margins: RectOffset::new(4.0, 4.0, 4.0, 4.0),
            },
            checkbox_bg: GuiImage {
                image_id: "checkbox_01".to_string(),
                margins: checkbox_bg_margins,
            },
            checkbox_bg_hovered: GuiImage {
                image_id: "checkbox_01_hovered".to_string(),
                margins: checkbox_bg_margins,
            },
            checkbox_bg_clicked: GuiImage {
                image_id: "checkbox_01_clicked".to_string(),
                margins: checkbox_bg_margins,
            },
            checkbox_bg_selected: GuiImage {
                image_id: "checkbox_01_selected".to_string(),
                margins: checkbox_bg_margins,
            },
            checkbox_bg_selected_hovered: GuiImage {
                image_id: "checkbox_01_selected_hovered".to_string(),
                margins: checkbox_bg_margins,
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuiSkins {
    pub default: Skin,
    pub window_title: Skin,
    pub module_list_entry: Skin,
    pub checkbox: Skin,
    pub checkbox_selected: Skin,
    pub header_label: Skin,
    pub warning_label: Skin,
    pub inactive_button: Skin,
    pub label_button: Skin,
    pub label_button_highlighted: Skin,
    pub condensed_button: Skin,
    pub condensed_button_inactive: Skin,
    pub big_editbox: Skin,
    pub slider_fix: Skin,
    pub theme: GuiTheme,
}

impl GuiSkins {
    pub const WINDOW_MARGIN_X: f32 = 25.0;
    pub const WINDOW_MARGIN_Y: f32 = 25.0;
    pub const ELEMENT_MARGIN: f32 = 8.0;
    pub const BUTTON_HEIGHT: f32 = 32.0;

    pub fn new(theme: GuiTheme) -> Self {
        let resources = storage::get::<Resources>();

        let window_bg = resources.images.get(&theme.window_bg.image_id).cloned().unwrap();

        let editbox_bg = resources.images.get(&theme.editbox_bg.image_id).unwrap();

        let button_bg = resources.images.get(&theme.button_bg.image_id).unwrap();
        let button_bg_hovered = resources.images.get(&theme.button_bg_hovered.image_id).unwrap();
        let button_bg_clicked = resources.images.get(&theme.button_bg_clicked.image_id).unwrap();
        let button_bg_inactive = resources.images.get(&theme.button_bg_inactive.image_id).unwrap();

        let checkbox_bg = resources.images.get(&theme.checkbox_bg.image_id).unwrap();
        let checkbox_bg_hovered = resources.images.get(&theme.checkbox_bg_hovered.image_id).unwrap();
        let checkbox_bg_clicked = resources.images.get(&theme.checkbox_bg_clicked.image_id).unwrap();
        let checkbox_bg_selected = resources.images.get(&theme.checkbox_bg_selected.image_id).unwrap();
        let checkbox_bg_selected_hovered = resources.images.get(&theme.checkbox_bg_selected_hovered.image_id).unwrap();

        let blank_image = resources.images.get("blank_image").unwrap();

        let default = {
            let window_style = root_ui()
                .style_builder()
                .background(window_bg.clone())
                .background_margin(theme.window_bg.margins)
                .margin(sub_offsets(theme.window_margins, theme.window_bg.margins))
                .build();

            let label_style = root_ui()
                .style_builder()
                .margin(theme.label_margins)
                .text_color(theme.text_color)
                .font_size(theme.font_size)
                .build();

            let button_style = root_ui()
                .style_builder()
                .margin(sub_offsets(theme.button_margins, theme.button_bg.margins))
                .background_margin(theme.button_bg.margins)
                .background(button_bg.clone())
                .background_hovered(button_bg_hovered.clone())
                .background_clicked(button_bg_clicked.clone())
                .text_color(theme.button_text_color)
                .font_size(theme.font_size)
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_bg.clone())
                .margin(sub_offsets(theme.editbox_margins, theme.editbox_bg.margins))
                .background_margin(theme.editbox_bg.margins)
                .text_color(theme.editbox_text_color)
                .font_size(theme.font_size)
                .build();

            let checkbox_style = root_ui()
                .style_builder()
                .background(checkbox_bg.clone())
                .background_hovered(checkbox_bg_hovered.clone())
                .background_clicked(checkbox_bg_clicked.clone())
                .build();

            let group_style = root_ui()
                .style_builder()
                .margin(theme.group_margins)
                .color(theme.group_border_color)
                .color_hovered(theme.group_border_color_hovered)
                .color_clicked(theme.group_border_color_clicked)
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(theme.scrollbar_color)
                .color_hovered(theme.scrollbar_color_hovered)
                .color_clicked(theme.scrollbar_color_clicked)
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(theme.scrollbar_handle_color)
                .color_hovered(theme.scrollbar_handle_color_hovered)
                .color_clicked(theme.scrollbar_handle_color_clicked)
                .build();

            Skin {
                window_style,
                label_style,
                button_style,
                editbox_style,
                checkbox_style,
                group_style,
                scrollbar_style,
                scrollbar_handle_style,
                scroll_multiplier: 10.0,
                ..root_ui().default_skin()
            }
        };

        let window_title = {
            let label_style = root_ui()
                .style_builder()
                .margin(theme.window_title_margins)
                .text_color(theme.window_title_color)
                .font_size(theme.window_title_size)
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let module_list_entry = {
            let group_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(Color::from_rgba(255, 255, 255, 255))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            Skin {
                group_style,
                scrollbar_style,
                scrollbar_handle_style,
                ..default.clone()
            }
        };

        let checkbox = {
            let button_style = root_ui()
                .style_builder()
                .background(checkbox_bg.clone())
                .background_hovered(checkbox_bg_hovered.clone())
                .background_clicked(checkbox_bg_clicked.clone())
                .background_margin(theme.checkbox_bg.margins)
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            let group_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            Skin {
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                group_style,
                ..default.clone()
            }
        };

        let checkbox_selected = {
            let button_style = root_ui()
                .style_builder()
                .background(checkbox_bg_selected.clone())
                .background_hovered(checkbox_bg_selected_hovered.clone())
                .background_clicked(checkbox_bg_clicked.clone())
                .background_margin(theme.checkbox_bg.margins)
                //.margin(RectOffset::new(-4.0, -4.0, -4.0, -4.0))
                .build();

            Skin {
                button_style,
                ..checkbox.clone()
            }
        };

        let header_label = {
            let label_style = root_ui()
                .style_builder()
                .margin(theme.header_label_margins)
                .text_color(theme.text_color)
                .font_size(theme.header_font_size)
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let warning_label = {
            let label_style = root_ui()
                .style_builder()
                .margin(theme.label_margins)
                .text_color(theme.warning_text_color)
                .font_size(theme.font_size)
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let inactive_button = {
            let button_style = root_ui()
                .style_builder()
                .margin(sub_offsets(theme.button_margins, theme.button_bg_inactive.margins))
                .background_margin(theme.button_bg_inactive.margins)
                .background(button_bg_inactive.clone())
                .background_hovered(button_bg_inactive.clone())
                .background_clicked(button_bg_inactive.clone())
                .text_color(theme.button_text_color_inactive)
                .font_size(theme.font_size)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let label_button = {
            let button_style = root_ui()
                .style_builder()
                .background(blank_image.clone())
                .background_hovered(blank_image.clone())
                .background_clicked(blank_image.clone())
                .margin(theme.label_margins)
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(theme.text_color)
                .font_size(theme.font_size)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let label_button_highlighted = {
            let button_style = root_ui()
                .style_builder()
                .background(blank_image.clone())
                .background_hovered(blank_image.clone())
                .background_clicked(blank_image.clone())
                .margin(theme.label_margins)
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(theme.highlight_text_color)
                .font_size(theme.font_size)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button = {
            let button_style = root_ui()
                .style_builder()
                .background(button_bg.clone())
                .background_hovered(button_bg_hovered.clone())
                .background_clicked(button_bg_clicked.clone())
                .margin(RectOffset::new(2.0, 2.0, -2.0, -2.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(theme.font_size)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button_inactive = {
            let button_style = root_ui()
                .style_builder()
                .background(button_bg_inactive.clone())
                .background_hovered(button_bg_inactive.clone())
                .background_clicked(button_bg_inactive.clone())
                .margin(RectOffset::new(2.0, 2.0, -2.0, -2.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(theme.font_size)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let big_editbox = {
            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_bg.clone())
                .margin(RectOffset::new(10.0, 10.0, 0.0, -4.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(theme.header_font_size)
                .build();

            Skin {
                editbox_style,
                ..default.clone()
            }
        };

        let slider_fix = {
            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_bg.clone())
                .margin(RectOffset::new(10.0, 10.0, 0.0, -4.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(theme.font_size)
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(COLOR_NONE)
                .color_hovered(COLOR_NONE)
                .color_clicked(COLOR_NONE)
                .build();

            Skin {
                editbox_style,
                scrollbar_style,
                scrollbar_handle_style,
                ..default.clone()
            }
        };

        GuiSkins {
            default,
            window_title,
            module_list_entry,
            checkbox,
            checkbox_selected,
            header_label,
            warning_label,
            inactive_button,
            label_button,
            label_button_highlighted,
            condensed_button,
            condensed_button_inactive,
            big_editbox,
            slider_fix,
            theme,
        }
    }
}

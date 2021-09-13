use crate::{
    gui::*,
};

#[derive(Debug, Clone)]
pub struct GuiSkins {
    pub default: Skin,
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
}

impl GuiSkins {
    pub const WINDOW_MARGIN_X: f32 = 25.0;
    pub const WINDOW_MARGIN_Y: f32 = 25.0;
    pub const ELEMENT_MARGIN: f32 = 4.0;
    pub const BUTTON_HEIGHT: f32 = 32.0;

    pub fn new() -> Self {
        let resources = storage::get::<Resources>();

        let panel_01 = resources.images.get("panel_01").cloned().unwrap();

        let editbox_01 = resources.images.get("editbox_01").unwrap();

        let btn_01 = resources.images.get("btn_01").unwrap();
        let btn_01_hover = resources.images.get("btn_01_hover").unwrap();
        let btn_01_click = resources.images.get("btn_01_click").unwrap();
        let btn_01_inactive = resources.images.get("btn_01_inactive").unwrap();

        let checkbox_01 = resources.images.get("checkbox_01").unwrap();
        let checkbox_01_hover = resources.images.get("checkbox_01_hover").unwrap();
        let checkbox_01_click = resources.images.get("checkbox_01_click").unwrap();
        let checkbox_01_selected = resources.images.get("checkbox_01_selected").unwrap();
        let checkbox_01_selected_hover = resources.images.get("checkbox_01_selected_hover").unwrap();

        let blank_image = resources.images.get("blank_image").unwrap();

        // let btn_02 = resources.images.get("btn_02").unwrap();
        // let btn_02_hover = resources.images.get("btn_02_hover").unwrap();
        // let btn_02_click = resources.images.get("btn_02_click").unwrap();
        //
        // let btn_03 = resources.images.get("btn_03").unwrap();
        // let btn_03_hover = resources.images.get("btn_03_hover").unwrap();
        // let btn_03_click = resources.images.get("btn_03_click").unwrap();

        let default = {
            let window_style = root_ui()
                .style_builder()
                .background(panel_01.clone())
                .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .margin(RectOffset::new(-27.0, -27.0, -27.0, -27.0))
                .build();

            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 4.0, 4.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 2.0, 2.0))
                .background_margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .margin(RectOffset::new(8.0, 8.0, 2.0, 2.0))
                .background_margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .background(btn_01.clone())
                .background_hovered(btn_01_hover.clone())
                .background_clicked(btn_01_click.clone())
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_01.clone())
                .margin(RectOffset::new(10.0, 10.0, 0.0, -4.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let checkbox_style = root_ui()
                .style_builder()
                .background(checkbox_01.clone())
                .background_hovered(checkbox_01_hover.clone())
                .background_clicked(checkbox_01_click.clone())
                .build();

            let group_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color(Color::from_rgba(0, 0, 0, 0))
                .color_hovered(Color::from_rgba(0, 0, 0, 0))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(58, 68,68, 255))
                .color_hovered(Color::from_rgba(58, 68,102, 255))
                .color_clicked(Color::from_rgba(58, 68,102, 255))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(58, 68,68, 255))
                .color_hovered(Color::from_rgba(58, 68,102, 255))
                .color_clicked(Color::from_rgba(58, 68,102, 255))
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

        let module_list_entry = {
            let group_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0, 0, 0))
                .color_hovered(Color::from_rgba(255, 255, 255, 255))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
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
                .background(checkbox_01.clone())
                .background_hovered(checkbox_01_hover.clone())
                .background_clicked(checkbox_01_click.clone())
                .background_margin(RectOffset::new(2.0, 2.0, 2.0, 2.0))
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
                .build();

            Skin {
                button_style,
                scrollbar_style,
                scrollbar_handle_style,
                ..default.clone()
            }
        };

        let checkbox_selected = {
            let button_style = root_ui()
                .style_builder()
                .background(checkbox_01_selected.clone())
                .background_hovered(checkbox_01_selected_hover.clone())
                .background_clicked(checkbox_01_click.clone())
                .background_margin(RectOffset::new(2.0, 2.0, 2.0, 2.0))
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
                .margin(RectOffset::new(0.0, 0.0, 5.0, 5.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(18)
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let warning_label = {
            let label_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(0.0, 0.0, 5.0, 5.0))
                .text_color(color::RED)
                .font_size(16)
                .build();

            Skin {
                label_style,
                ..default.clone()
            }
        };

        let inactive_button = {
            let button_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(8.0, 8.0, 2.0, 2.0))
                .background_margin(RectOffset::new(8.0, 8.0, 4.0, 4.0))
                .background(btn_01_inactive.clone())
                .background_hovered(btn_01_inactive.clone())
                .background_clicked(btn_01_inactive.clone())
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
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
                .margin(RectOffset::new(0.0, 0.0, 2.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(16)
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
                .margin(RectOffset::new(0.0, 0.0, 2.0, 0.0))
                .background_margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .text_color(Color::from_rgba(255, 255, 255, 255))
                .font_size(16)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button = {
            let button_style = root_ui()
                .style_builder()
                .background(btn_01.clone())
                .background_hovered(btn_01_hover.clone())
                .background_clicked(btn_01_click.clone())
                .margin(RectOffset::new(2.0, 2.0, -2.0, -2.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(16)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let condensed_button_inactive = {
            let button_style = root_ui()
                .style_builder()
                .background(btn_01_inactive.clone())
                .background_hovered(btn_01_inactive.clone())
                .background_clicked(btn_01_inactive.clone())
                .margin(RectOffset::new(2.0, 2.0, -2.0, -2.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(16)
                .build();

            Skin {
                button_style,
                ..default.clone()
            }
        };

        let big_editbox = {
            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_01.clone())
                .margin(RectOffset::new(10.0, 10.0, 0.0, -4.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(18)
                .build();

            Skin {
                editbox_style,
                ..default.clone()
            }
        };

        let slider_fix = {
            let editbox_style = root_ui()
                .style_builder()
                .background(editbox_01.clone())
                .margin(RectOffset::new(10.0, 10.0, 0.0, -4.0))
                .background_margin(RectOffset::new(4.0, 4.0, 4.0, 4.0))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size(16)
                .build();

            let scrollbar_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
                .build();

            let scrollbar_handle_style = root_ui()
                .style_builder()
                .color(Color::from_rgba(0, 0,0, 0))
                .color_hovered(Color::from_rgba(0, 0,0, 0))
                .color_clicked(Color::from_rgba(0, 0,0, 0))
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
        }
    }
}

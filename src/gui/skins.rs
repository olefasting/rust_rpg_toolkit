use macroquad::prelude::*;

use macroquad::ui::{root_ui, Skin};

#[derive(Debug, Clone)]
pub struct GuiSkins {
    pub scale: f32,
    pub default: Skin,
    pub inventory: Skin,
    pub character: Skin,
}

impl GuiSkins {
    pub fn new(scale: f32) -> Self {
        let default = {
            let window_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/MainPanel03.png"),
                    None,
                ))
                //.background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                .margin(RectOffset::new(15.0 * scale, 15.0 * scale, 15.0 * scale, 15.0 * scale))
                .build();

            let label_style = root_ui()
                .style_builder()
                // .margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size((16 as f32 * scale) as u16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button04.png"),
                    None,
                ))
                //.background_margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                .margin(RectOffset::new(15.0 * scale, 15.0 * scale, 5.0 * scale, 5.0 * scale))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button02.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button02.png"),
                    None,
                ))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size((16 as f32 * scale) as u16)
                .build();

            Skin {
                window_style,
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        let inventory = {
            let label_style = root_ui()
                .style_builder()
                // .margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size((12 as f32 * scale) as u16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button04.png"),
                    None,
                ))
                //.background_margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                .margin(RectOffset::new(8.0 * scale, 8.0 * scale, 2.0 * scale, 2.0 * scale))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button02.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button02.png"),
                    None,
                ))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size((12 as f32 * scale) as u16)
                .build();

            let group_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(15.0 * scale, 15.0 * scale, 5.0 * scale, 5.0 * scale))
                .build();

            Skin {
                label_style,
                button_style,
                group_style,
                ..default.clone()
            }
        };

        let character = {
            let label_style = root_ui()
                .style_builder()
                // .margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .margin(RectOffset::new(0.0 * scale, 0.0 * scale, 2.0 * scale, 0.0 * scale))
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size((12 as f32 * scale) as u16)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button11.png"),
                    None,
                ))
                //.background_margin(RectOffset::new(15.0, 15.0, 15.0, 15.0))
                .margin(RectOffset::new(8.0 * scale, 8.0 * scale, 2.0 * scale, 2.0 * scale))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button11.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button11.png"),
                    None,
                ))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size((12 as f32 * scale) as u16)
                .build();

            let group_style = root_ui()
                .style_builder()
                .margin(RectOffset::new(15.0 * scale, 15.0 * scale, 5.0 * scale, 5.0 * scale))
                .build();

            let editbox_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/wenrexa/Button11.png"),
                    None,
                ))
                .text_color(Color::from_rgba(200, 200, 160, 255))
                .font_size((12 as f32 * scale) as u16)
                .build();

            Skin {
                label_style,
                button_style,
                group_style,
                editbox_style,
                ..default.clone()
            }
        };

        GuiSkins {
            scale,
            default,
            inventory,
            character,
        }
    }
}

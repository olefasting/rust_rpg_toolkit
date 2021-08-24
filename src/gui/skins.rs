use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

#[derive(Debug, Clone)]
pub struct GuiSkins {
    pub scale: f32,
    pub default: Skin,
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
                .font_size(16)
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
                .font_size(16)
                .build();

            Skin {
                window_style,
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        GuiSkins {
            scale,
            default,
        }
    }
}

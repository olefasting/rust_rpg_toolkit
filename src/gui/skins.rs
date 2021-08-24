use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};
pub struct GuiSkins {
    pub main_menu: Skin,
}

impl GuiSkins {
    pub fn new() -> Self {
        let main_menu = {
            let label_style = root_ui()
                .style_builder()
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(255, 255, 160, 255))
                .font_size(24)
                .build();

            let button_style = root_ui()
                .style_builder()
                .background(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/button_background.png"),
                    None,
                ))
                //.background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
                //.margin(RectOffset::new(-40.0, -40.0, -40.0, -40.0))
                .background_hovered(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/button_background.png"),
                    None,
                ))
                .background_clicked(Image::from_file_with_format(
                    include_bytes!("../../assets/gui/button_background.png"),
                    None,
                ))
                // .font(include_bytes!("../../assets/gui/fonts/MinimalPixel v2.ttf"))
                // .unwrap()
                .text_color(Color::from_rgba(200, 200, 160, 255))
                //.reverse_background_z(true)
                .font_size(24)
                .build();

            Skin {
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };

        GuiSkins {
            main_menu,
        }
    }
}

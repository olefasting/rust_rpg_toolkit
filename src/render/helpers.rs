use macroquad::{
    prelude::*,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub fn draw_progress_bar(
    current_value: f32,
    max_value: f32,
    position: Vec2,
    length:f32,
    height: f32,
    color: Color,
    bg_color: Color,
    border: f32,
    alignment: HorizontalAlignment,
    text: Option<&str>,
    text_params: Option<TextParams>,
) {
    assert!(border * 2.0 < height && border * 2.0 < length, "Progress bar length and height must be greater than border * 2");
    {
        let coords = match alignment {
            HorizontalAlignment::Left => (
                position.x,
                position.y,
                position.x + length,
                position.y,
            ),
            HorizontalAlignment::Center => (
                position.x - length / 2.0,
                position.y,
                position.x + length / 2.0,
                position.y,
            ),
            HorizontalAlignment::Right => (
                position.x + length,
                position.y,
                position.x + length * 2.0,
                position.y,
            ),
        };
        draw_line(
            coords.0,
            coords.1,
            coords.2,
            coords.3,
            height,
            bg_color,
        );
    }
    {
        let coords = match alignment {
            HorizontalAlignment::Left => (
                position.x + border,
                position.y,
                position.x + (current_value / max_value) * length - border,
                position.y,
            ),
            HorizontalAlignment::Center => (
                position.x + border - length / 2.0,
                position.y,
                position.x + (current_value / max_value) * (length - border) - length / 2.0,
                position.y,
            ),
            HorizontalAlignment::Right => (
                position.x + border + length,
                position.y,
                position.x + (current_value / max_value) * (length - border) + length * 2.0,
                position.y,
            ),
        };
        draw_line(
            coords.0,
            coords.1,
            coords.2,
            coords.3,
            height - border * 2.0,
            color,
        );
    }
    {
        if let Some(text) = text {
            draw_aligned_text(
                text,
                position.x,
                position.y,
                HorizontalAlignment::Center,
                VerticalAlignment::Center,
                text_params.unwrap_or(TextParams {
                    font_size: height as u16,
                    ..Default::default()
                }),
            );
        }
    }
}

pub fn draw_aligned_text(text: &str, x: f32, y: f32, ha: HorizontalAlignment, va: VerticalAlignment, params: TextParams) {
    let measure = measure_text(
        text,
        Some(params.font),
        params.font_size,
        params.font_scale,
    );
    let x = match ha {
        HorizontalAlignment::Left => x,
        _ => {
            if ha == HorizontalAlignment::Center {
                x - (measure.width / 2.0)
            } else {
                x - measure.width
            }
        }
    };
    let y = match va {
        VerticalAlignment::Top => y + measure.height,
        VerticalAlignment::Center => y + measure.height / 2.0,
        _ => y,
    };

    draw_text_ex(text, x, y, params);
}

pub fn try_color_from_hex_string(str: &str) -> Option<Color> {
    let len = str.len();
    if len == 6 || (len == 7 && str.starts_with("#")) {
        let str = if len == 7 {
            str[1..7].to_string()
        } else {
            str.to_string()
        };
        let bytes = hex::decode(&str).unwrap();
        Some(Color::new(
            bytes[0] as f32 / 255.0,
            bytes[1] as f32 / 255.0,
            bytes[2] as f32 / 255.0,
            1.0,
        ))
    } else {
        None
    }
}

pub fn color_from_hex_string(str: &str) -> Color {
    try_color_from_hex_string(str)
        .expect("Color hex must be prefixed by a has, if it is not six characters long!")
}

use crate::prelude::macroquad::text::draw_text_ex;
use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub fn draw_text(
    text: &str,
    position: Vec2,
    ha: HorizontalAlignment,
    va: VerticalAlignment,
    params: TextParams,
) {
    let measure = get_text_measure(text, Some(params.font), params.font_size, params.font_scale);

    let x = match ha {
        HorizontalAlignment::Left => position.x,
        _ => {
            if ha == HorizontalAlignment::Center {
                position.x - (measure.width / 2.0)
            } else {
                position.x - measure.width
            }
        }
    };
    let y = match va {
        VerticalAlignment::Top => position.y + measure.height,
        VerticalAlignment::Center => position.y + measure.height / 2.0,
        _ => position.y,
    };

    draw_text_ex(text, x, y, params);
}

pub fn draw_progress_bar(
    current_value: f32,
    max_value: f32,
    position: Vec2,
    length: f32,
    height: f32,
    color: Color,
    bg_color: Color,
    border: f32,
    alignment: HorizontalAlignment,
    text: Option<&str>,
    text_params: Option<TextParams>,
) {
    assert!(
        border * 2.0 < height && border * 2.0 < length,
        "Progress bar length and height must be greater than border * 2"
    );
    {
        let coords = match alignment {
            HorizontalAlignment::Left => (position.x, position.y, position.x + length, position.y),
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
        draw_line(coords.0, coords.1, coords.2, coords.3, height, bg_color);
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
            draw_text(
                text,
                position,
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

pub fn color_from_hex_string(str: &str) -> Color {
    let str = if str.starts_with('#') {
        str[1..str.len()].to_string()
    } else {
        str.to_string()
    };

    let r = u8::from_str_radix(&str[0..2], 16).unwrap();
    let g = u8::from_str_radix(&str[2..4], 16).unwrap();
    let b = u8::from_str_radix(&str[4..6], 16).unwrap();
    let a = if str.len() > 6 {
        u8::from_str_radix(&str[6..8], 16).unwrap()
    } else {
        255
    };

    Color::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_hex_string_no_hash() {
        assert_eq!(
            color_from_hex_string("12ab6f"),
            Color::new(
                18 as f32 / 255.0,
                171 as f32 / 255.0,
                111 as f32 / 255.0,
                255 as f32 / 255.0,
            )
        );
    }

    #[test]
    fn test_color_from_hex_string_hash() {
        assert_eq!(
            color_from_hex_string("#12ab6f"),
            Color::new(
                18 as f32 / 255.0,
                171 as f32 / 255.0,
                111 as f32 / 255.0,
                255 as f32 / 255.0,
            )
        );
    }

    #[test]
    fn test_color_from_hex_string_alpha() {
        assert_eq!(
            color_from_hex_string("12ab6fb2"),
            Color::new(
                18 as f32 / 255.0,
                171 as f32 / 255.0,
                111 as f32 / 255.0,
                178 as f32 / 255.0,
            )
        );
    }
}

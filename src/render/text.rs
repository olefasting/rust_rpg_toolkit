use macroquad::prelude::*;

pub enum TextAlignment {
    Left,
    Right,
    Center,
}

pub fn draw_aligned_text(text: &str, x: f32, y: f32, alignment: TextAlignment, params: TextParams) {
    draw_text_ex(
        text,
        match alignment {
            TextAlignment::Left => x,
            _ => {
                let measure = measure_text(
                    text,
                    Some(params.font),
                    params.font_size,
                    params.font_scale,
                );
                if let TextAlignment::Center = alignment {
                    x - (measure.width / 2.0)
                } else {
                    x - measure.width
                }
            }
        },
        y,
        params,
    );
}

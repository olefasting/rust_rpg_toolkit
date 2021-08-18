use macroquad::prelude::*;

use crate::render::HorizontalAlignment;

pub fn draw_aligned_text(text: &str, x: f32, y: f32, alignment: HorizontalAlignment, params: TextParams) {
    let x = match alignment {
        HorizontalAlignment::Left => x,
        _ => {
            let measure = measure_text(
                text,
                Some(params.font),
                params.font_size,
                params.font_scale,
            );
            if let HorizontalAlignment::Center = alignment {
                x - (measure.width / 2.0)
            } else {
                x - measure.width
            }
        }
    };

    draw_text_ex(text, x, y, params);
}

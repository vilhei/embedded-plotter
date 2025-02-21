#![no_std]
use embedded_graphics::prelude::{Point, Size};
use u8g2_fonts::{
    fonts::{u8g2_font_4x6_tf, u8g2_font_4x6_tn, u8g2_font_6x13_mr},
    FontRenderer,
};

pub mod axis;
pub mod bar;
pub mod bar_line;
pub mod line;
pub mod scatter;

pub const DEFAULT_FONT: u8g2_fonts::FontRenderer = FontRenderer::new::<u8g2_font_4x6_tf>();
// pub const DEFAULT_FONT: u8g2_fonts::FontRenderer = FontRenderer::new::<u8g2_font_6x13_mr>();

/// Scales point from chart scale to display scale (e.g. to pixel coordinates for drawing)
pub fn scale_point(
    p: Point,
    display_size: &Size,
    origin: &Point,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
) -> Point {
    // TODO change arguments to some kind of struct config or similar to ease use
    let new_x = scale_value(p.x, x_min, x_max, origin.x, display_size.width as i32);
    let new_y = scale_value(
        p.y,
        y_min,
        y_max,
        display_size.height as i32 - origin.y,
        display_size.height as i32,
    );
    Point::new(new_x, new_y)
}

/// Make sure
pub fn scale_value(value: i32, old_min: i32, old_max: i32, min: i32, max: i32) -> i32 {
    // Will overflow absolute difference is higher than can fit in i32
    // let percentage_value = value / old_max - old_min;
    assert!(value >= old_min);
    assert!(value <= old_max);

    let old_range = old_max - old_min;
    let new_range = max - min;
    (value - old_min) * new_range / old_range + min
}

#[cfg(test)]
mod tests {
    // use super::*;
    use test_case::test_case;

    use crate::scale_value;

    #[test_case(5,0,10,0,20,10 ; "5 from 0-10 to 0-20")]
    #[test_case(50, 0, 10, 0, 160, 3)]
    fn test_scale_value(value: i32, old_min: i32, old_max: i32, min: i32, max: i32, expected: i32) {
        let result = scale_value(value, old_min, old_max, min, max);
        assert_eq!(expected, result);
    }
}

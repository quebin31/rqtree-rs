use delta_e::DE2000;
use image::Rgb;
use imageproc::rect::Rect as IRect;
use lab::Lab;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    pub upper_left: Point,
    pub lower_right: Point,
}

pub fn color_diff(lh: Rgb<u8>, rh: Rgb<u8>) -> f64 {
    let lh = Lab::from_rgb(&lh.0);
    let rh = Lab::from_rgb(&rh.0);

    DE2000::new(lh, rh) as f64
}

pub fn rect_to_irect(rect: Rect) -> IRect {
    IRect::at(
        rect.upper_left.x.try_into().unwrap(),
        rect.upper_left.y.try_into().unwrap(),
    )
    .of_size(
        (rect.lower_right.x - rect.upper_left.x + 1)
            .try_into()
            .unwrap(),
        (rect.lower_right.y - rect.upper_left.y + 1)
            .try_into()
            .unwrap(),
    )
}

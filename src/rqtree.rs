use crate::prim::{color_diff, rect_to_irect, Point, Rect};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;

#[derive(Debug, Clone)]
struct NodeData {
    rect: Rect,
    value: Option<Rgb<u8>>,
    upper_left: Box<Node>,
    upper_rigth: Box<Node>,
    lower_left: Box<Node>,
    lower_right: Box<Node>,
}

impl NodeData {
    fn new() -> Self {
        Self {
            rect: Default::default(),
            value: None,
            upper_left: Box::new(Node::None),
            upper_rigth: Box::new(Node::None),
            lower_left: Box::new(Node::None),
            lower_right: Box::new(Node::None),
        }
    }

    fn with_rect(rect: Rect) -> Self {
        Self {
            rect,
            ..Self::new()
        }
    }
}

#[derive(Debug, Clone)]
enum Node {
    None,
    Some(NodeData),
}

impl Node {
    fn null() -> Self {
        Node::None
    }

    fn leaf() -> Self {
        Node::Some(NodeData::new())
    }

    fn with_rect(rect: Rect) -> Self {
        Node::Some(NodeData::with_rect(rect))
    }
}

#[derive(Debug, Clone)]
pub struct RQTree {
    root: Box<Node>,
    umbral: f64,
}

impl RQTree {
    pub fn with_umbral(umbral: f64) -> Self {
        Self {
            root: Box::new(Node::null()),
            umbral,
        }
    }

    pub fn segment_image(&mut self, image: &RgbImage) -> RgbImage {
        self.root = Box::new(Node::leaf());

        if let Node::Some(node) = &mut *self.root {
            node.rect = Rect {
                upper_left: Point { x: 0, y: 0 },
                lower_right: Point {
                    x: image.dimensions().0 as usize,
                    y: image.dimensions().1 as usize,
                },
            };
        }

        let mut output = image.clone();
        Self::recursive_segment_image(&mut self.root, &image, &mut output, self.umbral);
        output
    }

    fn should_be_divided(rect: Rect, image: &RgbImage, umbral: f64) -> bool {
        let piv = image.get_pixel(rect.upper_left.x as u32, rect.upper_left.y as u32);

        for x in (rect.upper_left.x)..(rect.lower_right.x) {
            for y in (rect.upper_left.y)..(rect.lower_right.y) {
                let pix = image.get_pixel(x as u32, y as u32);
                if color_diff(*piv, *pix) > umbral {
                    return true;
                }
            }
        }

        false
    }

    fn get_piv_pixel(rect: Rect, image: &RgbImage) -> Rgb<u8> {
        *(image.get_pixel(rect.upper_left.x as u32, rect.upper_left.y as u32))
    }

    fn recursive_segment_image(
        node: &mut Node,
        image: &RgbImage,
        output: &mut RgbImage,
        umbral: f64,
    ) {
        let rect = if let Node::Some(node) = &*node {
            node.rect
        } else {
            panic!("Node is null!");
        };

        if !Self::should_be_divided(rect, &image, umbral) {
            if let Node::Some(node) = &mut *node {
                node.value = Some(Self::get_piv_pixel(rect, &image));
            }

            let irect = rect_to_irect(rect);
            draw_hollow_rect_mut(output, irect, Rgb([255_u8, 0, 0]));

            return;
        }

        let mid_x = (rect.lower_right.x - rect.upper_left.x) / 2 + rect.upper_left.x;
        let mid_y = (rect.lower_right.y - rect.upper_left.y) / 2 + rect.upper_left.y;

        let upper_left = Rect {
            upper_left: rect.upper_left,
            lower_right: Point { x: mid_x, y: mid_y },
        };

        let upper_right = Rect {
            upper_left: Point {
                x: mid_x,
                y: rect.upper_left.y,
            },
            lower_right: Point {
                x: rect.lower_right.x,
                y: mid_y,
            },
        };

        let lower_left = Rect {
            upper_left: Point {
                x: rect.upper_left.x,
                y: mid_y,
            },
            lower_right: Point {
                x: mid_x,
                y: rect.lower_right.y,
            },
        };

        let lower_right = Rect {
            upper_left: Point { x: mid_x, y: mid_y },
            lower_right: rect.lower_right,
        };

        if let Node::Some(node) = &mut *node {
            node.upper_left = Box::new(Node::with_rect(upper_left));
            Self::recursive_segment_image(&mut node.upper_left, image, output, umbral);

            node.upper_rigth = Box::new(Node::with_rect(upper_right));
            Self::recursive_segment_image(&mut node.upper_rigth, image, output, umbral);

            node.lower_left = Box::new(Node::with_rect(lower_left));
            Self::recursive_segment_image(&mut node.lower_left, image, output, umbral);

            node.lower_right = Box::new(Node::with_rect(lower_right));
            Self::recursive_segment_image(&mut node.lower_right, image, output, umbral);
        };
    }
}

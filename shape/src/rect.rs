use super::prelude::*;

pub struct RectangleParam {
    pub top_left: Position,
    pub size: Size,
}

pub struct RectangleFactory;

impl ShapeParam for RectangleParam {}
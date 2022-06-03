use super::prelude::*;

pub struct LineParam {
    pub from: Position,
    pub to: Position,
}

pub struct LineFactory;

impl ShapeParam for LineParam {}
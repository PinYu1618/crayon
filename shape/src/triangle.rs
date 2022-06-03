use super::prelude::*;

pub struct TriangleParam {
    pub p1: Position,
    pub p2: Position,
    pub p3: Position,
}

pub struct TriangleFactory;

impl ShapeParam for TriangleParam {}
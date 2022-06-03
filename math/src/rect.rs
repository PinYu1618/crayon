use crate::Position;
use crate::Size;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    position: Position,
    size: Size,
}

impl Rectangle {
    pub fn new(position: impl Into<Position>, size: impl Into<Size>) -> Self {
        Rectangle { position: position.into(), size: size.into() }
    }

    pub fn top_left(&self) -> Position {
        self.position
    }

    pub fn top_left_x(&self) -> f64 {
        self.position.x()
    }

    pub fn top_left_y(&self) -> f64 {
        self.position.y()
    }

    pub fn set_top_left(&mut self, position: impl Into<Position>) {
        self.position = position.into();
    }

    pub fn set_top_left_x(&mut self, x: impl Into<f64>) {
        self.position.set_x(x.into());
    }

    pub fn set_top_left_y(&mut self, y: impl Into<f64>) {
        self.position.set_y(y.into());
    }

    pub fn top_right(&self) -> Position {
        Position::from((self.top_left_x() + self.width(), self.top_left_y()))
    }

    pub fn top_right_x(&self) -> f64 {
        self.top_left_x() + self.width()
    }

    pub fn width(&self) -> f64 {
        self.size.width()
    }

    pub fn height(&self) -> f64 {
        self.size.height()
    }

    pub fn set_width(&mut self, width: impl Into<f64>) {
        self.size.set_width(width.into());
    }

    pub fn set_height(&mut self, height: impl Into<f64>) {
        self.size.set_height(height.into());
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, width: impl Into<f64>, height: impl Into<f64>) {
        self.size.set_width(width.into());
        self.size.set_height(height.into());
    }

    pub fn contains(&self, point: impl Into<Position>) -> bool {
        let position: Position = point.into();
        position.x() >= self.top_left_x()
            && position.x() <= self.top_right_x()
            && position.y() >= self.top_left_y()
            && position.y() <= self.top_left_y() + self.height()
    }

    pub fn contains_rect(&self, rect: &Rectangle) -> bool {
        let p1 = rect.top_left();
        let p2 = (p1.x() + rect.width(), p1.y() + rect.height());
        self.contains(p1) && self.contains(p2)
    }

    pub fn intersects(&self, rect: &Rectangle) -> bool {
        !(rect.top_left_x() > self.top_right_x()
            || self.top_left_x() > rect.top_right_x()
            || rect.top_left_y() > (self.top_left_y() + self.height())
            || self.top_left_y() > (rect.top_left_y() + rect.height()))
    }

    pub fn join_with_rectangle(&mut self, other: &Rectangle) {
        if other.top_left_x() < self.top_left_x() {
            self.set_width(self.top_right_x() - other.top_left_x());
            self.set_top_left_x(other.top_left_x());
        }
        if other.top_left_y() < self.top_left_y() {
            self.set_height(self.height() + self.top_left_y() - other.top_left_y());
            self.set_top_left_y(other.top_left_y());
        }
        if other.top_right_x() > self.top_right_x() {
            self.set_width(other.top_right_x() - self.top_left_x());
        }
        if other.top_left_y() + other.height() > self.top_left_y() + self.height() {
            self.set_height(other.top_left_y() + other.height() - self.top_left_y());
        }
    }

    /// Extends this rectangle to cover the given point.
    pub fn join_with_point(&mut self, point: &Position) {
        if point.x() < self.top_left_x() {
            self.set_width(self.top_right_x() - point.x());
            self.set_top_left_x(point.x());
        }
        if point.y() < self.top_left_y() {
            self.set_height(self.height() + self.top_left_y() - point.y());
            self.set_top_left_y(point.y());
        }
        if point.x() > self.top_right_x() {
            self.set_width(point.x() - self.top_left_x());
        }
        if point.y() > self.top_left_y() + self.height() {
            self.set_height(point.y() - self.top_left_y());
        }
    }
}

impl From<(Position, Size)> for Rectangle {
    fn from(t: (Position, Size)) -> Self {
        Rectangle::new(t.0, t.1)
    }
}

impl From<(f64, f64, f64, f64)> for Rectangle {
    fn from(t: (f64, f64, f64, f64)) -> Self {
        Rectangle::new((t.0, t.1), (t.2, t.3))
    }
}

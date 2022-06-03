#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    pub fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    pub fn x_add(&mut self, val: f64) {
        self.x += val;
    }

    pub fn y_add(&mut self, val: f64) {
        self.y += val;
    }
}

impl core::ops::Add for Position {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl core::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl From<(f64, f64)> for Position {
    fn from(val: (f64, f64)) -> Self {
        Position::new(val.0, val.1)
    }
}

impl From<(u16, u16)> for Position {
    fn from(val: (u16, u16)) -> Self {
        Position::new(val.0.into(), val.1.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let point = Position::default();
        assert_eq!(point, Position{ x: 0.0, y: 0.0 });
    }
}
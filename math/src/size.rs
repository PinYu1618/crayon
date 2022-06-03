#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    width: f64,
    height: f64,
}

impl Size {
    pub fn new(width: f64, height: f64) -> Self {
        Size { width, height }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl core::ops::Div<f64> for Size {
    type Output = Size;

    fn div(mut self, rhs: f64) -> Self::Output {
        self.width /= rhs;
        self.height /= rhs;
        self
    }
}

impl From<f64> for Size {
    fn from(t: f64) -> Self {
        Size::new(t, t)
    }
}

impl From<(f64, f64)> for Size {
    fn from(s: (f64, f64)) -> Size {
        Size::new(s.0, s.1)
    }
}
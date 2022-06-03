pub mod prelude;

pub struct Shape{
    pub positions: Vec<Position>,
}

pub trait ShapeParam {}

pub trait ShapeFactory {
    type R: ShapeParam;
    
    fn create(param: Self::R) -> Shape;
}

mod line;
mod rect;
mod triangle;

use utils::Position;

pub use self::line::*;
pub use self::rect::*;
pub use self::triangle::*;
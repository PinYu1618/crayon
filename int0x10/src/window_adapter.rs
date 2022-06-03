use utils::Position;

pub trait WindowAdapter {
    fn mouse_position(&mut self) -> Position;
    fn clear_screan(&mut self);
}
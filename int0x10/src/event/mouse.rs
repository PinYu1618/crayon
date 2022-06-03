use crate::prelude::*;

pub trait MouseHandler: Sized {
    fn on_click();
}

pub struct ClickEvent {
    pub position: Position,
}
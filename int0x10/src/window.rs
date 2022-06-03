use utils::Size;
use crate::WindowAdapter;
use std::sync::Arc;
use core::sync::atomic::AtomicBool;

pub struct Window<A: WindowAdapter>{
    closed: bool,
    updated: bool,
    clipboard_updated: bool,
    redrawn: Arc<AtomicBool>,
    size: Size,
    adapter: A,
}

impl<A: WindowAdapter> Window<A> {
    pub fn render(&mut self) {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }
}
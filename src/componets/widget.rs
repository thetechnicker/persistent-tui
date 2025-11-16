use ratatui::Frame;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Widget {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Layout {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Layout {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    pub fn get_possition(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}

type WidgetType = Rc<RefCell<dyn Widget>>;

pub enum Component {
    Widget(WidgetType),
    ListView(Rc<[Component]>),
    GridView(Rc<[Rc<[Component]>]>),
    Floating(WidgetType),
}

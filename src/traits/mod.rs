mod group;
mod input;
mod widget_base;
mod widget_ext;

pub use group::*;
pub use input::*;
pub use widget_base::*;
pub use widget_ext::*;

use crate::widget::Widget;

#[derive(Debug)]
pub struct ElementIter {
    list: web_sys::HtmlCollection,
    index: u32,
}

pub trait IsElementIterable {
    fn iter(&self) -> ElementIter;
}

impl IsElementIterable for web_sys::HtmlCollection {
    fn iter(&self) -> ElementIter {
        ElementIter {
            list: self.clone(),
            index: 0,
        }
    }
}

impl Iterator for ElementIter {
    type Item = Widget;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.list.item(self.index) {
            self.index += 1;
            Some(Widget::from_elem(item))
        } else {
            None
        }
    }
}


use crate::{enums::*, widget::Widget};

use crate::prelude::{WidgetBase, WidgetExt, InputExt};
use crate::group::PARENTS;


#[derive(Clone)]
pub struct Input {
    inner: Widget,
}

impl WidgetBase for Input {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Input);
        unsafe {
            if let Some(last) = PARENTS.last() {
                last.append(&inner);
            }
        }
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self { inner: widget.clone() }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Input {}

impl InputExt for Input {}

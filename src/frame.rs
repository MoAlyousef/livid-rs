use crate::{enums::*, widget::Widget};

use crate::group::PARENTS;
use crate::prelude::*;

#[derive(Clone)]
pub struct Frame {
    inner: Widget,
}

impl WidgetBase for Frame {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_style(Style::TextAlign, "center");
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
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Frame {}

use crate::prelude::*;
use crate::{enums::*, widget::Widget};

#[derive(Debug, Clone)]
pub struct Frame {
    inner: Widget,
}

impl WidgetBase for Frame {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_style(Style::TextAlign, "center");
        Self { inner }
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

#[derive(Debug, Clone)]
pub struct Label {
    inner: Widget,
}

impl WidgetBase for Label {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        Self { inner }
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

impl WidgetExt for Label {}

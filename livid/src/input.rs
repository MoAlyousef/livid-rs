use crate::prelude::{InputExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};

#[derive(Clone)]
pub struct Input {
    inner: Widget,
}

impl WidgetBase for Input {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Input);
        crate::group::Group::current_attach(&inner);
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

impl WidgetExt for Input {}

impl InputExt for Input {}

#[derive(Clone)]
pub struct TextArea {
    inner: Widget,
}

impl WidgetBase for TextArea {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Textarea);
        inner.set_style(Style::Resize, "none");
        crate::group::Group::current_attach(&inner);
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

impl WidgetExt for TextArea {}

impl InputExt for TextArea {}

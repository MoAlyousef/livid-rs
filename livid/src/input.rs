use crate::prelude::{InputExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};

#[derive(Debug, Clone)]
pub struct Input {
    inner: Widget,
}

impl WidgetBase for Input {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Input);
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

impl WidgetExt for Input {}

impl InputExt for Input {}

#[derive(Debug, Clone)]
pub struct TextArea {
    inner: Widget,
}

impl WidgetBase for TextArea {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Textarea);
        inner.set_style(Style::Resize, "none");
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

impl WidgetExt for TextArea {}

impl InputExt for TextArea {}

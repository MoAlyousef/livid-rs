use crate::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};

pub struct Group {
    inner: Widget,
}

impl WidgetBase for Group {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.begin();
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

impl WidgetExt for Group {}

impl GroupExt for Group {}

pub struct Column {
    inner: Widget,
}

impl WidgetBase for Column {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_class_name("flex-container");
        inner.set_style(Style::Display, "flex");
        inner.set_style(Style::FlexDirection, "column");
        inner.set_style(Style::AlignContent, "space-between");
        inner.begin();
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

impl WidgetExt for Column {}

impl GroupExt for Column {}

pub struct Row {
    inner: Widget,
}

impl WidgetBase for Row {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        inner.set_class_name("flex-container");
        inner.set_style(Style::Display, "flex");
        inner.set_style(Style::FlexDirection, "row");
        inner.set_style(Style::AlignContent, "space-between");
        inner.begin();
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

impl WidgetExt for Row {}

impl GroupExt for Row {}

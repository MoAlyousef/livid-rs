use crate::{enums::*, widget::Widget};

use crate::prelude::{WidgetBase, WidgetExt, GroupExt};

pub(crate) static mut PARENTS: Vec<Widget> = vec![];

pub struct Group {
    inner: Widget,
}

impl WidgetBase for Group {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        unsafe {
            if let Some(last) = PARENTS.last() {
                last.append(&inner);
            }
            PARENTS.push(inner.clone());
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
        unsafe {
            if let Some(last) = PARENTS.last() {
                last.append(&inner);
            }
            PARENTS.push(inner.clone());
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
        unsafe {
            if let Some(last) = PARENTS.last() {
                last.append(&inner);
            }
            PARENTS.push(inner.clone());
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

impl WidgetExt for Row {}

impl GroupExt for Row {}
use crate::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};
use std::cell::RefCell;
// use wasm_bindgen::JsValue;

thread_local! {
    static PARENTS: RefCell<Vec<Widget>> = RefCell::from(vec![]);
}

pub struct Group {
    inner: Widget,
}

impl Group {
    pub fn current_attach(w: &Widget) {
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(w);
            }
        });
    }
    pub fn set_current(w: &Widget) {
        PARENTS.with(|p| {
            p.borrow_mut().push(w.clone());
        });
    }
    pub fn group_begin(w: &Widget) {
        PARENTS.with(|p| p.borrow_mut().push(w.clone()));
    }
    pub fn group_end() {
        PARENTS.with(|p| p.borrow_mut().pop());
    }
}

impl WidgetBase for Group {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        Group::current_attach(&inner);
        Group::set_current(&inner);
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
        Group::current_attach(&inner);
        Group::set_current(&inner);
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
        Group::current_attach(&inner);
        Group::set_current(&inner);
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

impl WidgetExt for Row {}

impl GroupExt for Row {}

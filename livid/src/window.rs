use crate::document::Document;
use crate::prelude::{GroupExt, WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) static HAS_WINDOW: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct Window {
    inner: Widget,
}

impl Window {
    pub fn new_relative(x: i32, y: i32) -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, &format!("{}%", x));
        s.inner.set_style(Style::Height, &format!("{}%", y));
        s
    }

    /// Check if the document has a main livid window
    pub fn has_window() -> bool {
        HAS_WINDOW.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl WidgetBase for Window {
    fn default() -> Self {
        let outer = Widget::new(WidgetType::Div);
        outer.set_style(Style::Display, "table");
        outer.set_style(Style::Position, "absolute");
        outer.set_style(Style::Left, "0");
        outer.set_style(Style::Top, "0");
        outer.set_style(Style::Width, "100%");
        outer.set_style(Style::Height, "100%");
        let middle = Widget::new(WidgetType::Div);
        outer.append(&middle);
        middle.set_style(Style::Display, "table-cell");
        middle.set_style(Style::VerticalAlign, "middle");
        let inner = Widget::new(WidgetType::Div);
        middle.append(&inner);
        inner.set_style(Style::MarginLeft, "auto");
        inner.set_style(Style::MarginRight, "auto");
        inner.begin();
        HAS_WINDOW.store(true, Ordering::Relaxed);
        Self { inner }
    }

    fn new<T: Into<Option<&'static str>>>(_x: i32, _y: i32, w: i32, h: i32, title: T) -> Self {
        if let Some(title) = title.into() {
            Document::get().set_title(title);
        }
        let s = Self::default();
        s.inner.set_style(Style::Width, &format!("{}", w));
        s.inner.set_style(Style::Height, &format!("{}", h));
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

impl WidgetExt for Window {
    fn set_label(&self, label: &str) {
        Document::get().set_title(label);
    }
}

impl GroupExt for Window {}

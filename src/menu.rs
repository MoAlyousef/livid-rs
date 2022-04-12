use crate::group::PARENTS;
use crate::prelude::*;
use crate::{enums::*, widget::Widget};

#[derive(Clone)]
pub struct Choice {
    inner: Widget,
}

impl Choice {
    pub fn add_choice(&self, choice: &str) {
        let opt = Widget::new(WidgetType::Option);
        opt.set_text_content(Some(choice));
        self.inner.append(&opt);
    }
}

impl WidgetBase for Choice {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Select);
        inner.set_style(Style::TextAlign, "center");
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
        });
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

impl WidgetExt for Choice {}

#[derive(Clone)]
pub struct NavBar {
    inner: Widget,
}

impl NavBar {
    pub fn add_choice(&self, choice: &str) -> crate::misc::Link {
        let opt = Widget::new(WidgetType::Li);
        opt.set_style(Style::Display, "block");
        opt.set_style(Style::Padding, "8px 16px");
        opt.set_style(Style::TextDecoration, "none");
        let link = Widget::new(WidgetType::A);
        link.set_text_content(Some(choice));
        opt.append(&link);
        self.inner.append(&opt);
        unsafe { crate::misc::Link::from_widget(&link) }
    }
}

impl WidgetBase for NavBar {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Ul);
        inner.set_style(Style::ListStyleType, "none");
        inner.set_style(Style::Margin, "0");
        inner.set_style(Style::Padding, "0");
        inner.set_style(Style::Width, "200px");
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
        });
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

impl WidgetExt for NavBar {}

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

impl WidgetExt for Choice {}

#[derive(Clone)]
pub struct NavBar {
    inner: Widget,
}

impl NavBar {
    pub fn add_choice(&self, choice: &str) -> crate::misc::Link {
        let opt = Widget::new(WidgetType::Li);
        let link = Widget::new(WidgetType::A);
        link.set_style(Style::Display, "block");
        link.set_style(Style::Padding, "8px 16px");
        link.set_style(Style::TextDecoration, "none");
        link.set_text_content(Some(choice));
        link.add_callback(Event::MouseOver, |l| {
            l.set_style(Style::BackgroundColor, "#555");
            l.set_style(Style::Color, "white");
        });
        link.add_callback(Event::MouseLeave, |l| {
            l.set_style(Style::BackgroundColor, "rgba(0,0,0,0)");
            l.set_style(Style::Color, "black");
        });
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
        inner.set_style(Style::BackgroundColor, "#f1f1f1");
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

impl WidgetExt for NavBar {}


#[derive(Clone)]
pub struct MenuBar {
    inner: Widget,
}

impl MenuBar {
    pub fn add_choice(&self, choice: &str) -> crate::misc::Link {
        let opt = Widget::new(WidgetType::Li);
        opt.set_style(Style::Float, "left");
        opt.set_style(Style::Display, "inline");
        opt.set_style(Style::Padding, "8px 16px");
        opt.set_style(Style::TextDecoration, "none");
        opt.set_style(Style::TextAlign, "center");
        let link = Widget::new(WidgetType::A);
        link.set_style(Style::Display, "inline");
        link.set_style(Style::Padding, "8px 16px");
        link.set_style(Style::TextDecoration, "none");
        link.set_text_content(Some(choice));
        link.add_callback(Event::MouseOver, |l| {
            l.set_style(Style::BackgroundColor, "#555");
            l.set_style(Style::Color, "white");
        });
        link.add_callback(Event::MouseLeave, |l| {
            l.set_style(Style::BackgroundColor, "rgba(0,0,0,0)");
            l.set_style(Style::Color, "black");
        });
        opt.append(&link);
        self.inner.append(&opt);
        unsafe { crate::misc::Link::from_widget(&link) }
    }
}

impl WidgetBase for MenuBar {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Ul);
        inner.set_style(Style::ListStyleType, "none");
        inner.set_style(Style::Margin, "0");
        inner.set_style(Style::Padding, "0");
        inner.set_style(Style::Overflow, "hidden");
        inner.set_style(Style::Border, "1px solid #e7e7e7");
        inner.set_style(Style::BackgroundColor, "#f1f1f1");
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

impl WidgetExt for MenuBar {}
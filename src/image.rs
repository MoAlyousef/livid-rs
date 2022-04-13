use crate::group::PARENTS;
use crate::prelude::*;
use crate::{enums::*, widget::Widget};
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct Image {
    inner: Widget,
}

impl Image {
    pub fn set_src(&self, href: &str) {
        let elem: web_sys::HtmlImageElement = JsValue::from((*self.inner).clone()).into();
        elem.set_src(href);
    }
    pub fn with_src(self, href: &str) -> Self {
        self.set_src(href);
        self
    }
    pub fn set_size(&self, w: u32, h: u32) {
        let elem: web_sys::HtmlImageElement = JsValue::from((*self.inner).clone()).into();
        elem.set_width(w);
        elem.set_height(h);
    }
    pub fn with_size(self, w: u32, h: u32) -> Self {
        self.set_size(w, h);
        self
    }
}

impl WidgetBase for Image {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Img);
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

impl WidgetExt for Image {}
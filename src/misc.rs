use crate::group::PARENTS;
use crate::prelude::*;
use crate::{enums::*, widget::Widget};
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct Link {
    inner: Widget,
}

impl Link {
    pub fn set_href(&self, href: &str) {
        let elem: web_sys::HtmlLinkElement = JsValue::from((*self.inner).clone()).into();
        elem.set_href(href);
    }
    pub fn with_href(self, href: &str) -> Self {
        self.set_href(href);
        self
    }
}

impl WidgetBase for Link {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::A);
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

impl WidgetExt for Link {}

impl GroupExt for Link {}

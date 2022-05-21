use crate::prelude::*;
use crate::{enums::*, widget::Widget};
use wasm_bindgen::{JsValue, JsCast};

#[derive(Debug, Clone)]
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
        crate::group::Group::current_attach(&inner);
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

impl WidgetExt for Link {}

impl GroupExt for Link {}

#[derive(Debug, Clone)]
pub struct Break {
    inner: Widget,
}

impl WidgetBase for Break {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Br);
        crate::group::Group::current_attach(&inner);
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

impl WidgetExt for Break {}

#[derive(Debug, Clone)]
pub struct Canvas {
    inner: Widget,
}

pub type CanvasContext = web_sys::CanvasRenderingContext2d;

impl Canvas {
    pub fn draw<F: 'static + FnMut(CanvasContext)>(&self, mut cb: F) {
        let inner = self.inner.elem().clone();
        let canvas: web_sys::HtmlCanvasElement = inner
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        cb(context)
    }
}

impl WidgetBase for Canvas {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Canvas);
        crate::group::Group::current_attach(&inner);
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

impl WidgetExt for Canvas {}

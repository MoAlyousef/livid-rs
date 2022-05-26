use crate::prelude::*;
use crate::{enums::*, widget::Widget};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::closure::Closure;

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
    pub fn draw<F: 'static + FnMut(&Self, CanvasContext)>(&self, mut cb: F) {
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
        cb(self, context)
    }
}

impl WidgetBase for Canvas {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Canvas);
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

impl SvgDrawer for CanvasContext {
    fn draw_svg(&self, svg: &str, x: f64, y: f64, w: f64, h: f64) {
        let bt = crate::utils::btoa(svg).unwrap();
        let svg = String::from("data:image/svg+xml;base64,") + &bt;
        let doc = crate::document::Document::get();
        let img = doc.create_element("IMG").unwrap_throw();
        let img: &web_sys::HtmlImageElement = img.dyn_ref().unwrap_throw();
        let ctx = self.clone();
        let imgg = img.clone();
        let cb1 = Closure::wrap(Box::new(move || {
            ctx.draw_image_with_html_image_element_and_dw_and_dh(&imgg, x, y, w, h).unwrap_throw();
        }) as Box<dyn FnMut()>);
        img.set_onload(Some(cb1.as_ref().unchecked_ref()));
        img.set_src(&svg);
        cb1.forget();
    }
}

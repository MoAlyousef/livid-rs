use crate::document::Document;
use crate::enums::*;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
/// An Html Element wrapper
pub struct Widget {
    elem: web_sys::Element,
}

impl Widget {
    /// Create a new Widget
    pub fn new(typ: WidgetType) -> Self {
        let doc = Document::get();
        let elem = doc.create_element(&typ.to_str()).unwrap_throw();
        doc.body().unwrap_throw().append_child(&elem).unwrap_throw();
        Self { elem }
    }
    /// Delete a widget
    pub fn delete(w: Widget) {
        w.elem.set_outer_html("");
        drop(w.elem);
    }
    /// Create a new widget from an existing Element
    pub fn from_elem(elem: web_sys::Element) -> Self {
        Self { elem }
    }
    /// Create a new widget from an existing Element
    pub fn elem(&self) -> web_sys::Element {
        self.elem.clone()
    }
    /// Create a widget struct from an id
    pub fn from_id(id: &str) -> Option<Self> {
        Document::get()
            .get_element_by_id(id)
            .map(|elem| Self { elem })
    }
    /// Add a callback
    pub fn add_callback<F: 'static + FnMut(&Self)>(&self, event: Event, mut cb: F) {
        let e = self.clone();
        let cb1 = Closure::wrap(Box::new(move || {
            cb(&e);
        }) as Box<dyn FnMut()>);
        self.elem
            .add_event_listener_with_callback(&event.to_str(), cb1.as_ref().unchecked_ref())
            .unwrap_throw();
        cb1.forget();
    }
    /// Set a specific style
    pub fn set_style(&self, style: Style, val: &str) {
        let style_elem: web_sys::HtmlStyleElement = JsValue::from(self.elem.clone()).into();
        let css = style_elem.style();
        css.set_property(style.to_str(), val).unwrap_throw();
    }
    /// Get a specific style
    pub fn style(&self, s: Style) -> String {
        let style_elem: web_sys::HtmlStyleElement = JsValue::from(self.elem.clone()).into();
        let css = style_elem.style();
        css.get_property_value(s.to_str()).unwrap_throw()
    }
    /// Append a widget
    pub fn append(&self, other: &Widget) {
        self.elem.append_child(&other.elem).unwrap_throw();
    }
    /// Remove a widget
    pub fn remove(&self, other: &Widget) {
        self.elem.remove_child(&other.elem).unwrap_throw();
    }
}

impl Deref for Widget {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl DerefMut for Widget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elem
    }
}

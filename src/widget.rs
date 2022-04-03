use crate::events::{get_event_str, Event};
use crate::styles::{get_style_str, Style};
use crate::types::{get_element_str, WidgetType};
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Get the global window
pub fn window() -> web_sys::Window {
    web_sys::window().expect("No global window found!")
}

/// Get the global document
pub fn document() -> web_sys::Document {
    window().document().expect("No document found!")
}

/// Get the global docuement's body
pub fn body() -> web_sys::HtmlElement {
    document().body().expect("No body found!")
}

/// An Html Element wrapper
#[derive(Debug, Clone)]
pub struct Widget {
    elem: web_sys::Element,
}

impl Widget {
    /// Create a new Widget
    pub fn new(typ: WidgetType) -> Self {
        let typ = get_element_str(typ);
        let doc = document();
        let elem = doc.create_element(typ).unwrap();
        doc.body().unwrap().append_child(&elem).unwrap();
        Self { elem }
    }
    /// Create a new widget from an existing Element
    pub fn from(elem: web_sys::Element) -> Self {
        Self { elem }
    }
    /// Add a callback
    pub fn add_callback<F: 'static + FnMut(&Self)>(&self, event: Event, mut cb: F) {
        let e = self.clone();
        let cb1 = Closure::wrap(Box::new(move || {
            cb(&e);
        }) as Box<dyn FnMut()>);
        let event = get_event_str(event);
        self.elem
            .add_event_listener_with_callback(event, cb1.as_ref().unchecked_ref())
            .unwrap();
        cb1.forget();
    }
    /// Set a specific style
    pub fn set_style(&self, style: Style, val: &str) {
        let style = get_style_str(style);
        let style_elem: web_sys::HtmlStyleElement = JsValue::from(self.elem.clone()).into();
        let css = style_elem.style();
        css.set_property(style, val).unwrap();
    }
    /// Get a specific style
    pub fn style(&self, s: Style) -> String {
        let style = get_style_str(s);
        let style_elem: web_sys::HtmlStyleElement = JsValue::from(self.elem.clone()).into();
        let css = style_elem.style();
        css.get_property_value(style).unwrap()
    }
    /// Append a widget
    pub fn append(&self, other: &Widget) {
        self.elem.append_child(&other.elem).unwrap();
    }
    /// Remove a widget
    pub fn remove(&self, other: &Widget) {
        self.elem.remove_child(&other.elem).unwrap();
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

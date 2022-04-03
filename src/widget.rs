use crate::events::{get_event_str, Event};
use crate::styles::{get_style_str, Style};
use crate::types::{get_element_str, WidgetType};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local! {
    pub static WINDOW: RefCell<web_sys::Window> = RefCell::from(web_sys::window().unwrap());
    pub static DOCUMENT: RefCell<web_sys::Document> = RefCell::from(WINDOW.with(|w| w.clone()).borrow().document().unwrap());
}

#[derive(Debug, Clone)]
pub struct Widget {
    elem: web_sys::Element,
}

impl Widget {
    pub fn new(typ: WidgetType) -> Self {
        let typ = get_element_str(typ);
        let doc = DOCUMENT.with(|d| d.clone());
        let elem = doc.borrow().create_element(typ).unwrap();
        doc.borrow().body().unwrap().append_child(&elem).unwrap();
        Self { elem }
    }
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
    pub fn set_style(&self, style: Style, val: &str) {
        let style = get_style_str(style);
        let style_elem: web_sys::HtmlStyleElement = JsValue::from(self.elem.clone()).into();
        let css = style_elem.style();
        css.set_property(style, val).unwrap();
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

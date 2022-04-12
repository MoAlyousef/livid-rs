use wasm_bindgen::prelude::*;

/// Get the global window
fn window() -> web_sys::Window {
    web_sys::window().expect("No global window found!")
}

/// Get the global document
fn document() -> web_sys::Document {
    window().document().expect("No document found!")
}

#[derive(Clone, Copy)]
pub struct Document;

impl Document {
    /// Get the global document
    pub fn get() -> web_sys::Document {
        document()
    }

    /// Get the global docuement's body
    pub fn body() -> web_sys::HtmlElement {
        Self::get().body().expect("No body found!")
    }

    /// Get the head
    pub fn head() -> web_sys::HtmlHeadElement {
        Self::get().head().expect("No head element found!")
    }

    /// add a link
    pub fn add_css_link(href: &str) {
        let link: web_sys::HtmlLinkElement =
            JsValue::from(Document::get().create_element("link").unwrap()).into();
        link.set_rel("stylesheet");
        link.set_type("text/css");
        link.set_href(href);
        Self::head().append_child(&link).unwrap();
    }
}

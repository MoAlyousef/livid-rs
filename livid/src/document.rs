use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Copy)]
pub struct Document;

impl Document {
    /// Get the global document
    pub fn get() -> web_sys::Document {
        Self::window().document().expect_throw("No document found!")
    }

    /// Get the global docuement's body
    pub fn body() -> web_sys::HtmlElement {
        Self::get().body().expect_throw("No body found!")
    }

    /// Get the head
    pub fn head() -> web_sys::HtmlHeadElement {
        Self::get().head().expect_throw("No head element found!")
    }

    /// Get the global window
    pub fn window() -> web_sys::Window {
        web_sys::window().expect_throw("No global window found!")
    }

    /// add a link
    pub fn add_css_link(href: &str) {
        let link: web_sys::HtmlLinkElement = Document::get()
            .create_element("link")
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();
        link.set_rel("stylesheet");
        link.set_type("text/css");
        link.set_href(href);
        Self::head().append_child(&link).unwrap_throw();
    }
}

use crate::enums::*;
use crate::traits::WidgetExt;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

pub trait InputExt: WidgetExt {
    fn value(&self) -> String {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.clone().value()
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.clone().value()
        }
    }
    fn set_value(&self, s: &str) {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.set_value(s)
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.set_value(s)
        }
    }
    fn placeholder(&self) -> String {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.clone().placeholder()
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.clone().placeholder()
        }
    }
    fn set_placeholder(&self, s: &str) {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.set_placeholder(s)
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.set_placeholder(s)
        }
    }
    fn readonly(&self) -> bool {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.clone().read_only()
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.clone().read_only()
        }
    }
    fn set_readonly(&self, val: bool) {
        let c = self.inner();
        let tag = c.tag_name();
        if tag == "INPUT" {
            let elem = c.dyn_ref::<web_sys::HtmlInputElement>().unwrap_throw();
            elem.set_read_only(val)
        } else {
            let elem = c.dyn_ref::<web_sys::HtmlTextAreaElement>().unwrap_throw();
            elem.set_read_only(val)
        }
    }
    /// Sets the widget's color
    fn set_text_color(&self, color: Color) {
        self.inner().set_style(Style::Color, &color.to_str())
    }
    /// Returns the widget's label color
    fn text_color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::Color)).unwrap_throw()
    }
    fn set_text_size(&self, size: u8) {
        self.inner().set_style(Style::FontSize, &size.to_string());
    }
    fn text_size(&self) -> u8 {
        self.inner().style(Style::FontSize).parse().unwrap_throw()
    }
    fn set_text_font(&self, font: &str) {
        self.inner().set_style(Style::Font, font);
    }
    fn text_font(&self) -> String {
        self.inner().style(Style::Font)
    }
    fn set_text_align(&self, align: TextAlign) {
        self.inner().set_style(Style::TextAlign, &align.to_str());
    }
    fn set_direction(&self, dir: Direction) {
        self.inner().set_style(Style::Direction, &dir.to_str());
    }
}

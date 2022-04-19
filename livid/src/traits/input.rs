use crate::enums::*;
use crate::traits::WidgetExt;
use wasm_bindgen::JsCast;

pub trait InputExt: WidgetExt {
    fn value(&self) -> String {
        let c = self.inner();
        let elem: &web_sys::HtmlInputElement = c.dyn_ref().unwrap();
        elem.clone().value()
    }
    fn set_value(&self, s: &str) {
        let c = self.inner();
        let elem: &web_sys::HtmlInputElement = c.dyn_ref().unwrap();
        elem.set_value(s);
    }
    /// Sets the widget's color
    fn set_text_color(&self, color: Color) {
        self.inner().set_style(Style::Color, &color.to_str())
    }
    /// Returns the widget's label color
    fn text_color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::Color)).unwrap()
    }
    fn set_text_size(&self, size: u8) {
        self.inner().set_style(Style::FontSize, &size.to_string());
    }
    fn text_size(&self) -> u8 {
        self.inner().style(Style::FontSize).parse().unwrap()
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

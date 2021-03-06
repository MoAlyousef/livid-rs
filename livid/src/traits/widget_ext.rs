use crate::{
    enums::*,
    traits::{GroupExt, WidgetBase},
};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

/// Defines the methods implemented by all widgets
pub trait WidgetExt: WidgetBase {
    /// Set the callback for the widget
    fn add_callback<F: 'static + FnMut(&Self)>(&self, event: Event, mut cb: F)
    where
        Self: Sized,
    {
        self.inner().add_callback(event, move |w| unsafe {
            cb(&Self::from_widget(w));
        });
    }
    fn set_id(&self, id: &str) {
        self.inner().set_id(id);
    }
    fn with_id(self, id: &str) -> Self
    where
        Self: Sized,
    {
        self.set_id(id);
        self
    }
    fn set_class(&self, class: &str) {
        self.inner().set_class_name(class);
    }
    fn with_class(self, class: &str) -> Self
    where
        Self: Sized,
    {
        self.set_class(class);
        self
    }
    fn class(&self) -> String {
        self.inner().class_name()
    }
    /// Sets the widget's label
    fn set_label(&self, title: &str) {
        self.inner().set_text_content(Some(title))
    }
    fn with_label(self, title: &str) -> Self
    where
        Self: Sized,
    {
        self.set_label(title);
        self
    }
    /// Set to position x, y
    fn set_pos(&self, x: i32, y: i32) {
        let inner = self.inner();
        inner.set_style(Style::Left, &x.to_string());
        inner.set_style(Style::Top, &y.to_string());
    }
    /// Set to dimensions width and height
    fn set_size(&self, width: i32, height: i32) {
        let inner = self.inner();
        inner.set_style(Style::Width, &width.to_string());
        inner.set_style(Style::Height, &height.to_string());
    }
    /// set the size on construction
    fn with_size(self, w: i32, h: i32) -> Self
    where
        Self: Sized,
    {
        self.set_size(w, h);
        self
    }
    /// Redraws a widget, necessary for resizing and changing positions
    fn redraw(&self) {
        self.hide();
        self.show();
    }
    /// Shows the widget
    fn show(&self) {
        self.inner().set_style(Style::Display, "block");
    }
    /// Hides the widget
    fn hide(&self) {
        self.inner().set_style(Style::Display, "none");
    }
    /// Returns the x coordinate of the widget
    fn x(&self) -> i32 {
        self.inner().style(Style::Left).parse().unwrap_throw()
    }
    /// Returns the y coordinate of the widget
    fn y(&self) -> i32 {
        self.inner().style(Style::Top).parse().unwrap_throw()
    }
    /// Returns the width of the widget
    fn w(&self) -> i32 {
        self.inner().client_width()
    }
    /// Returns the height of the widget
    fn h(&self) -> i32 {
        self.inner().client_height()
    }
    /// Returns the label of the widget
    fn label(&self) -> Option<String> {
        self.inner().text_content()
    }
    /// Returns the widget color
    fn color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::BackgroundColor)).unwrap_throw()
    }
    /// Sets the widget's color
    fn set_color(&self, color: Color) {
        self.inner()
            .set_style(Style::BackgroundColor, &color.to_str())
    }
    /// Sets the widget's color
    fn set_label_color(&self, color: Color) {
        self.inner().set_style(Style::Color, &color.to_str())
    }
    /// Returns the widget's label color
    fn label_color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::Color)).unwrap_throw()
    }
    fn set_label_size(&self, size: u8) {
        self.inner().set_style(Style::FontSize, &size.to_string());
    }
    fn label_size(&self) -> u8 {
        self.inner().style(Style::FontSize).parse().unwrap_throw()
    }
    fn set_label_font(&self, font: &str) {
        self.inner().set_style(Style::Font, font);
    }
    fn label_font(&self) -> String {
        self.inner().style(Style::Font)
    }
    /// do callback
    fn do_callback(&self, event: Event) {
        let c = self.inner();
        let elem: &web_sys::EventTarget = c.dyn_ref().unwrap_throw();
        elem.dispatch_event(&web_sys::Event::new(&event.to_str()).unwrap_throw())
            .unwrap_throw();
    }
    fn set_margin(&self, size: i32) {
        self.inner().set_style(Style::Margin, &size.to_string());
    }
    fn margin(&self) -> i32 {
        self.inner().style(Style::Margin).parse().unwrap_throw()
    }
    fn set_padding(&self, size: i32) {
        self.inner().set_style(Style::Padding, &size.to_string());
    }
    fn padding(&self) -> i32 {
        self.inner().style(Style::Padding).parse().unwrap_throw()
    }
    fn set_frame(&self, frame: FrameType) {
        match frame {
            FrameType::FlatBox => {
                self.inner().set_style(Style::Border, "none");
                self.inner().set_style(Style::BorderRadius, "0px");
            }
            FrameType::RFlatBox => {
                self.inner().set_style(Style::Border, "none");
                self.inner().set_style(Style::BorderRadius, "8px");
            }
            FrameType::RoundBox => {
                self.inner().set_style(Style::Border, "none");
                self.inner().set_style(Style::BorderRadius, "50%");
            }
            FrameType::FlatFrame => {
                self.inner().set_style(Style::BorderRadius, "0px");
            }
            FrameType::RFlatFrame => {
                self.inner().set_style(Style::BorderRadius, "8px");
            }
            FrameType::RoundFrame => {
                self.inner().set_style(Style::BorderRadius, "50%");
            }
        }
    }
    fn parent(&self) -> Option<Box<dyn GroupExt>> {
        if let Some(parent) = self.inner().parent_element() {
            Some(Box::new(unsafe {
                crate::group::Group::from_widget(&crate::widget::Widget::from_elem(parent))
            }))
        } else {
            None
        }
    }
}

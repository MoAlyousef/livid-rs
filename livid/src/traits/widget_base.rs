use crate::{enums::*, widget::Widget};

pub trait WidgetBase {
    fn default() -> Self where Self: Sized;
    fn default_fill() -> Self where Self: Sized {
        let s = Self::default();
        s.inner().set_style(Style::Width, "100%");
        s.inner().set_style(Style::Height, "100%");
        s
    }
    /// Construct a new widget
    fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self
    where
        Self: Sized,
    {
        let s = Self::default();
        s.inner().set_text_content(title.into());
        if crate::window::Window::has_window() {
            s.inner().set_style(Style::Position, "relative");
        } else {
            s.inner().set_style(Style::Position, "absolute");
        }
        s.inner().set_style(Style::Left, &x.to_string());
        s.inner().set_style(Style::Top, &y.to_string());
        s.inner().set_style(Style::Width, &w.to_string());
        s.inner().set_style(Style::Height, &h.to_string());
        s
    }
    /// Create a typed widget from `widget::Widget`
    /// # Safety
    /// The types must be compatible
    unsafe fn from_widget(widget: &Widget) -> Self where Self: Sized;
    /// inner
    fn inner(&self) -> Widget;
}

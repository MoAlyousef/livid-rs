use crate::group::PARENTS;
use crate::{widget::Widget, enums::*};
use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use wasm_bindgen::prelude::*;

/// Get the global window
fn window() -> web_sys::Window {
    web_sys::window().expect("No global window found!")
}

/// Get the global document
fn document() -> web_sys::Document {
    window().document().expect("No document found!")
}

/// Error types returned by livid + wrappers of std errors
#[derive(Debug)]
#[non_exhaustive]
pub enum LividError {
    /// i/o error
    IoError(io::Error),
    /// Utf-8 conversion error
    Utf8Error(FromUtf8Error),
    /// Null string conversion error
    NullError(std::ffi::NulError),
    /// Internal livid error
    Internal(LividErrorKind),
    /// Error using an erroneous env variable
    EnvVarError(std::env::VarError),
    /// Parsing error
    ParseIntError(std::num::ParseIntError),
    /// Unknown error
    Unknown(String),
}

unsafe impl Send for LividError {}
unsafe impl Sync for LividError {}

/// Error kinds enum for `LividError`
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum LividErrorKind {
    /// Failed to run the application
    FailedToRun,
    /// Failed to initialize the multithreading
    FailedToLock,
    /// Failed to set the general scheme of the application
    FailedToSetScheme,
    /// Failed operation, mostly unknown reason!
    FailedOperation,
    /// System resource (file, image) not found
    ResourceNotFound,
    /// Image format error when opening an image of an unsupported format
    ImageFormatError,
    /// Error filling table
    TableError,
    /// Error due to printing
    PrintError,
    /// Invalid color
    InvalidColor,
}

impl std::error::Error for LividError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LividError::IoError(err) => Some(err),
            LividError::NullError(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for LividError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LividError::IoError(ref err) => err.fmt(f),
            LividError::NullError(ref err) => err.fmt(f),
            LividError::Internal(ref err) => write!(f, "An internal error occurred {:?}", err),
            LividError::EnvVarError(ref err) => write!(f, "An env var error occurred {:?}", err),
            LividError::Utf8Error(ref err) => {
                write!(f, "A UTF8 conversion error occurred {:?}", err)
            }
            LividError::ParseIntError(ref err) => {
                write!(f, "An int parsing error occurred {:?}", err)
            }
            LividError::Unknown(ref err) => write!(f, "An unknown error occurred {:?}", err),
        }
    }
}

impl From<io::Error> for LividError {
    fn from(err: io::Error) -> LividError {
        LividError::IoError(err)
    }
}

impl From<std::ffi::NulError> for LividError {
    fn from(err: std::ffi::NulError) -> LividError {
        LividError::NullError(err)
    }
}

impl From<std::env::VarError> for LividError {
    fn from(err: std::env::VarError) -> LividError {
        LividError::EnvVarError(err)
    }
}

impl From<std::string::FromUtf8Error> for LividError {
    fn from(err: std::string::FromUtf8Error) -> LividError {
        LividError::Utf8Error(err)
    }
}

impl From<std::num::ParseIntError> for LividError {
    fn from(err: std::num::ParseIntError) -> LividError {
        LividError::ParseIntError(err)
    }
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


#[derive(Debug)]
pub struct ElementIter {
    list: web_sys::HtmlCollection,
    index: u32
}


pub trait IsElementIterable {
    fn iter(&self) -> ElementIter;
}

impl IsElementIterable for web_sys::HtmlCollection {
    fn iter(&self) -> ElementIter {
        ElementIter {
            list: self.clone(),
            index: 0
        }
    }
}

impl Iterator for ElementIter {
    type Item = Widget;
    fn next( &mut self ) -> Option< Self::Item > {
        if let Some(item) = self.list.item(self.index) {
            self.index += 1;
            Some(Widget::from_elem(item))
        } else {
            None
        }
    }
}

pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn to_hex_str(&self) -> String {
        let (r, g, b) = (self.0, self.1, self.2);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
    /// Return a Color from a hex color format (`#xxxxxx`)
    pub fn from_hex_str(col: &str) -> Result<Color, LividError> {
        if !col.starts_with('#') || col.len() != 7 {
            Err(LividError::Internal(LividErrorKind::InvalidColor))
        } else {
            Ok(Color::from_hex(u32::from_str_radix(&col[1..7], 16)?))
        }
    }
    pub fn from_hex(val: u32) -> Color {
        let (r, g, b) = crate::utils::hex2rgb(val);
        Color(r, g, b)
    }
}

pub trait WidgetBase {
    fn default() -> Self;
    fn default_fill() -> Self;
    /// Construct a new widget
    fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self
    where
        Self: Sized,
    {
        let s = Self::default();
        s.inner().set_text_content(title.into());
        unsafe {
            if crate::window::HAS_WINDOW {
                s.inner().set_style(Style::Position, "relative");
            } else {
                s.inner().set_style(Style::Position, "absolute");
            }
        }
        s.inner().set_style(Style::Left, &x.to_string());
        s.inner().set_style(Style::Top, &y.to_string());
        s.inner().set_style(Style::Width, &w.to_string());
        s.inner().set_style(Style::Height, &h.to_string());
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self;
    /// inner
    fn inner(&self) -> Widget;
}

/// Defines the methods implemented by all widgets
pub trait WidgetExt: WidgetBase {
    /// Set the callback for the widget
    fn add_callback<F: 'static + FnMut(&Self)>(&self, event: Event, mut cb: F)
    where
        Self: Sized,
    {
        self.inner().add_callback(event, move |w| {
            unsafe { cb(&Self::from_widget(w)); }
        });
    }
    fn set_id(&self, id: &str) {
        self.inner().set_id(id);
    }
    fn with_id(self, id: &str) -> Self where Self:Sized {
        self.set_id(id);
        self
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
    fn with_size(self, w: i32, h: i32) -> Self where Self: Sized {
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
        self.inner().style(Style::Left).parse().unwrap()
    }
    /// Returns the y coordinate of the widget
    fn y(&self) -> i32 {
        self.inner().style(Style::Top).parse().unwrap()
    }
    /// Returns the width of the widget
    fn w(&self) -> i32 {
        self.inner().style(Style::Width).parse().unwrap()
    }
    /// Returns the height of the widget
    fn h(&self) -> i32 {
        self.inner().style(Style::Height).parse().unwrap()
    }
    /// Returns the label of the widget
    fn label(&self) -> Option<String> {
        self.inner().text_content()
    }
    /// Returns the widget color
    fn color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::BackgroundColor)).unwrap()
    }
    /// Sets the widget's color
    fn set_color(&self, color: Color) {
        self.inner()
            .set_style(Style::BackgroundColor, &color.to_hex_str())
    }
    /// Sets the widget's color
    fn set_label_color(&self, color: Color) {
        self.inner().set_style(Style::Color, &color.to_hex_str())
    }
    /// Returns the widget's label color
    fn label_color(&self) -> Color {
        Color::from_hex_str(&self.inner().style(Style::Color)).unwrap()
    }
    fn set_label_size(&self, size: u8) {
        self.inner().set_style(Style::FontSize, &size.to_string());
    }
    fn label_size(&self) -> u8 {
        self.inner().style(Style::FontSize).parse().unwrap()
    }
    fn set_label_font(&self, font: &str) {
        self.inner().set_style(Style::Font, font);
    }
    fn label_font(&self) -> String {
        self.inner().style(Style::Font)
    }
    /// do callback
    fn do_callback(&self, event: Event) {
        let elem: web_sys::EventTarget = JsValue::from((*self.inner()).clone()).into();
        elem.dispatch_event(&web_sys::Event::new(event.to_str()).unwrap())
            .unwrap();
    }
}

pub trait GroupExt: WidgetExt {
    fn begin(&self) {
        unsafe {
            PARENTS.push(self.inner().clone());
        }
    }
    fn end(&self) {
        unsafe {
            PARENTS.pop();
        }
    }
    fn add<W: WidgetExt>(&self, widget: &W) {
        self.inner().append(&widget.inner());
    }
    fn remove<W: WidgetExt>(&self, widget: &W) {
        self.inner().remove(&widget.inner());
    }
}

pub trait InputExt: WidgetExt {
    fn value(&self) -> String {
        let elem: web_sys::HtmlInputElement = JsValue::from((*self.inner()).clone()).into();
        elem.value()
    }
}

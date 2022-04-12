use crate::group::PARENTS;
use crate::prelude::{WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};
use wasm_bindgen::JsValue;

#[derive(Clone)]
pub struct Button {
    inner: Widget,
}

impl Button {
    pub fn disabled(&self) -> bool {
        let elem: web_sys::HtmlButtonElement = JsValue::from((*self.inner).clone()).into();
        elem.disabled()
    }
    pub fn set_disabled(&self, val: bool) {
        let elem: web_sys::HtmlButtonElement = JsValue::from((*self.inner).clone()).into();
        elem.set_disabled(val);
    }
}

impl WidgetBase for Button {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Button);
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

impl WidgetExt for Button {}

#[derive(Clone)]
pub struct RadioButton {
    inner: Widget,
}

impl RadioButton {
    pub fn set_name(&self, name: &str) {
        if let Some(btn) = self.inner.first_element_child() {
            btn.set_attribute("name", name).unwrap();
        }
    }
    pub fn with_name(self, name: &str) -> Self {
        self.set_name(name);
        self
    }
    pub fn checked(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.checked()
        } else {
            false
        }
    }
    pub fn set_checked(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.set_checked(val);
        }
    }
    pub fn disabled(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.disabled()
        } else {
            false
        }
    }
    pub fn set_disabled(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.set_disabled(val);
        }
    }
}

impl WidgetBase for RadioButton {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        let btn = Widget::new(WidgetType::Input);
        btn.set_attribute("type", "radio").unwrap();
        let label = Widget::new(WidgetType::Label);
        inner.append(&btn);
        inner.append(&label);
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
    fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self
    where
        Self: Sized,
    {
        let inner = {
            let inner = Widget::new(WidgetType::Div);
            let btn = Widget::new(WidgetType::Input);
            btn.set_attribute("type", "radio").unwrap();
            let label = Widget::new(WidgetType::Label);
            label.set_text_content(title.into());
            inner.append(&btn);
            inner.append(&label);
            PARENTS.with(|p| {
                if let Some(last) = p.borrow().last() {
                    last.append(&inner);
                }
            });
            inner
        };
        if crate::window::HAS_WINDOW.load(std::sync::atomic::Ordering::Relaxed) {
            inner.set_style(Style::Position, "relative");
        } else {
            inner.set_style(Style::Position, "absolute");
        }
        inner.set_style(Style::Left, &x.to_string());
        inner.set_style(Style::Top, &y.to_string());
        inner.set_style(Style::Width, &w.to_string());
        inner.set_style(Style::Height, &h.to_string());
        Self { inner }
    }
}

impl WidgetExt for RadioButton {
    fn set_label(&self, title: &str) {
        if let Some(label) = self.inner.last_element_child() {
            label.set_text_content(Some(title));
        }
    }
}

#[derive(Clone)]
pub struct CheckButton {
    inner: Widget,
}

impl CheckButton {
    pub fn set_name(&self, name: &str) {
        if let Some(btn) = self.inner.first_element_child() {
            btn.set_attribute("name", name).unwrap();
        }
    }
    pub fn with_name(self, name: &str) -> Self {
        self.set_name(name);
        self
    }
    pub fn checked(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.checked()
        } else {
            false
        }
    }
    pub fn set_checked(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.set_checked(val);
        }
    }
    pub fn disabled(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.disabled()
        } else {
            false
        }
    }
    pub fn set_disabled(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: web_sys::HtmlInputElement = JsValue::from(btn).into();
            elem.set_disabled(val);
        }
    }
}

impl WidgetBase for CheckButton {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        let btn = Widget::new(WidgetType::Input);
        btn.set_attribute("type", "checkbox").unwrap();
        let label = Widget::new(WidgetType::Label);
        inner.append(&btn);
        inner.append(&label);
        PARENTS.with(|p| {
            if let Some(last) = p.borrow().last() {
                last.append(&inner);
            }
        });
        Self { inner }
    }
    fn default_fill() -> Self {
        let s = Self::default();
        s.inner.set_style(Style::Width, "100%");
        s.inner.set_style(Style::Height, "100%");
        s
    }
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
        }
    }
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
    fn new<T: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self
    where
        Self: Sized,
    {
        let inner = {
            let inner = Widget::new(WidgetType::Div);
            let btn = Widget::new(WidgetType::Input);
            btn.set_attribute("type", "checkbox").unwrap();
            let label = Widget::new(WidgetType::Label);
            label.set_text_content(title.into());
            inner.append(&btn);
            inner.append(&label);
            PARENTS.with(|p| {
                if let Some(last) = p.borrow().last() {
                    last.append(&inner);
                }
            });
            inner
        };
        if crate::window::HAS_WINDOW.load(std::sync::atomic::Ordering::Relaxed) {
            inner.set_style(Style::Position, "relative");
        } else {
            inner.set_style(Style::Position, "absolute");
        }
        inner.set_style(Style::Left, &x.to_string());
        inner.set_style(Style::Top, &y.to_string());
        inner.set_style(Style::Width, &w.to_string());
        inner.set_style(Style::Height, &h.to_string());
        Self { inner }
    }
}

impl WidgetExt for CheckButton {
    fn set_label(&self, title: &str) {
        if let Some(label) = self.inner.last_element_child() {
            label.set_text_content(Some(title));
        }
    }
}

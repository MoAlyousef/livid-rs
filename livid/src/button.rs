use crate::prelude::{WidgetBase, WidgetExt};
use crate::{enums::*, widget::Widget};
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Clone)]
pub struct Button {
    inner: Widget,
}

impl Button {
    pub fn disabled(&self) -> bool {
        let elem: &web_sys::HtmlButtonElement = self.inner.dyn_ref().unwrap_throw();
        elem.disabled()
    }
    pub fn set_disabled(&self, val: bool) {
        let elem: &web_sys::HtmlButtonElement = self.inner.dyn_ref().unwrap_throw();
        elem.set_disabled(val);
    }
}

impl WidgetBase for Button {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Button);
        crate::group::Group::current_attach(&inner);
        Self { inner }
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

#[derive(Debug, Clone)]
pub struct RadioButton {
    inner: Widget,
}

impl RadioButton {
    pub fn set_name(&self, name: &str) {
        if let Some(btn) = self.inner.first_element_child() {
            btn.set_attribute("name", name).unwrap_throw();
        }
    }
    pub fn with_name(self, name: &str) -> Self {
        self.set_name(name);
        self
    }
    pub fn checked(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.checked()
        } else {
            false
        }
    }
    pub fn set_checked(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.set_checked(val);
        }
    }
    pub fn disabled(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.disabled()
        } else {
            false
        }
    }
    pub fn set_disabled(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.set_disabled(val);
        }
    }
}

impl WidgetBase for RadioButton {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        let btn = Widget::new(WidgetType::Input);
        btn.set_attribute("type", "radio").unwrap_throw();
        let label = Widget::new(WidgetType::Label);
        inner.append(&btn);
        inner.append(&label);
        crate::group::Group::current_attach(&inner);
        Self { inner }
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

impl WidgetExt for RadioButton {
    fn set_label(&self, title: &str) {
        if let Some(label) = self.inner.last_element_child() {
            label.set_text_content(Some(title));
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckButton {
    inner: Widget,
}

impl CheckButton {
    pub fn set_name(&self, name: &str) {
        if let Some(btn) = self.inner.first_element_child() {
            btn.set_attribute("name", name).unwrap_throw();
        }
    }
    pub fn with_name(self, name: &str) -> Self {
        self.set_name(name);
        self
    }
    pub fn checked(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.checked()
        } else {
            false
        }
    }
    pub fn set_checked(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.set_checked(val);
        }
    }
    pub fn disabled(&self) -> bool {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.disabled()
        } else {
            false
        }
    }
    pub fn set_disabled(&self, val: bool) {
        if let Some(btn) = self.inner.first_element_child() {
            let elem: &web_sys::HtmlInputElement = btn.dyn_ref().unwrap_throw();
            elem.set_disabled(val);
        }
    }
}

impl WidgetBase for CheckButton {
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div);
        let btn = Widget::new(WidgetType::Input);
        btn.set_attribute("type", "checkbox").unwrap_throw();
        let label = Widget::new(WidgetType::Label);
        inner.append(&btn);
        inner.append(&label);
        crate::group::Group::current_attach(&inner);
        Self { inner }
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

impl WidgetExt for CheckButton {
    fn set_label(&self, title: &str) {
        if let Some(label) = self.inner.last_element_child() {
            label.set_text_content(Some(title));
        }
    }
}

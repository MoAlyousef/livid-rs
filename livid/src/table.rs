use crate::prelude::*;
use crate::{enums::*, widget::Widget};
use wasm_bindgen::UnwrapThrowExt;

fn table() -> Widget {
    Widget::new(WidgetType::Table)
}

fn tbody() -> Widget {
    Widget::new(WidgetType::Tbody)
}

fn tr() -> Widget {
    Widget::new(WidgetType::Tr)
}

fn td() -> Widget {
    Widget::new(WidgetType::Td)
}

#[derive(Debug, Clone)]
pub struct TableView {
    inner: Widget,
}

impl TableView {
    pub fn set_view(&self, data: &[Vec<&str>]) {
        let tb = tbody();
        self.inner.append(&tb);
        for item in data.iter() {
            let tr = tr();
            tb.append(&tr);

            for sub in item.iter() {
                let td = td();
                td.set_attribute("width", "100").unwrap_throw();
                td.set_text_content(Some(sub));
                tr.append(&td);
            }
        }
    }
}

impl WidgetBase for TableView {
    fn default() -> Self {
        let inner = table();
        inner.set_style(Style::TextAlign, "center");
        inner.set_attribute("border", "1").unwrap_throw();
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

impl WidgetExt for TableView {}

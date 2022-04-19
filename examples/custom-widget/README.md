# Custom livid widget

This crate shows how to create a custom widget. It requires the implementation of `livid::prelude::{WidgetBase, WidgetExt}` traits.
The minimal boilerplate needed is:
```rust
use livid::prelude::*;
use livid::{enums::*, group::Group, widget::Widget};

#[derive(Debug, Clone)]
pub struct MyWidget {
    inner: Widget,
    // other fiels
}


impl WidgetBase for MyWidget {
    // implement a default constructor
    fn default() -> Self {
        let inner = Widget::new(WidgetType::Div); // can be any WidgetType
        // Customize style etc
        inner.set_style(Style::Color, Color::Red);
        // Attach to the current parent in scope
        Group::current_attach(&inner);
        Self { inner, /* other fields */ }
    }
    // Implement a from_widget constructor
    unsafe fn from_widget(widget: &Widget) -> Self {
        Self {
            inner: widget.clone(),
            /* other fields */
        }
    }
    // Tell livid what inner resolves to
    fn inner(&self) -> Widget {
        self.inner.clone()
    }
}

// livid calls WidgetBase::inner and applies the default implemented functions, you can specialize any specific function to your liking
impl WidgetExt for MyWidget {}
```
# livid-rs

livid is a lightweight frontend Rust crate.

## Requirements
- The wasm32-unknown-unknown target:
- `rustup target add wasm32-unknown-unknown`

## Usage
- Install the trunk crate (to simplify building and bundling your web app):
- `cargo install trunk`

- Create an index.html file in your root directory:
```html
<html>
  <head>
  </head>
</html>
```
* You can add links to CSS files/urls, and use Widget::set_class_name() to benefit from CSS styling.

- Create a project:
```toml
[dependencies]
livid = { git = "https://github.com/MoAlyousef/livid-rs" }
```

```rust
use livid::{Event, Style, Widget, WidgetType, document};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let count = Rc::from(RefCell::from(0));

    let btn_inc = Widget::new(WidgetType::Button);
    btn_inc.set_text_content(Some("Increment"));
    btn_inc.set_style(Style::Color, "green");
    btn_inc.add_callback(Event::Click, {
        let cnt = count.clone();
        move |_| {
            *cnt.borrow_mut() += 1;
            let result = document().get_element_by_id("result").unwrap();
            result.set_text_content(Some(&cnt.borrow().to_string()));
    }});

    let div = Widget::new(WidgetType::Div);
    div.append_child(&btn_inc).unwrap();

    let result = Widget::new(WidgetType::Div);
    result.set_id("result");
    result.set_text_content(Some("0"));
    result.set_style(Style::FontSize, "22px");
}
```

- Build and serve using Trunk:
- `trunk build`
- `trunk serve`
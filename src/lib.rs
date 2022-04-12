/*!
# livid

livid is a lightweight frontend Rust crate for creating web apps via webassembly.
- Thin wrapper around web-sys
- No vdom.
- No macros!

## Requirements
- The wasm32-unknown-unknown target:

`rustup target add wasm32-unknown-unknown`

## Usage
- Install the dister crate (to simplify building and bundling your web app):

`cargo install dister`

* You can add links to CSS files/urls, and use Widget::set_class_name() to benefit from CSS styling.

- Create a project:
```toml
[dependencies]
livid = "0.2"
```

In your Rust source file:

```rust,no_run
use livid::{enums::*, prelude::*, *};

fn div() -> widget::Widget {
    widget::Widget::new(WidgetType::Div)
}

fn btn(i: i32) -> widget::Widget {
    let btn = widget::Widget::new(WidgetType::Button);
    let (label, col) = if i > 0 {
        ("Increment", "Green")
    } else {
        ("Decrement", "Red")
    };
    btn.set_text_content(Some(label));
    btn.set_style(Style::Color, col);
    btn.add_callback(Event::Click, move |_| {
        let result = widget::Widget::from_id("result").unwrap();
        let mut old: i32 = result.text_content().unwrap().parse().unwrap();
        old += i;
        result.set_text_content(Some(&old.to_string()));
    });
    btn
}

fn main() {
    document::Document::get().set_title("Counter");

    let btn_inc = btn(1);
    let btn_dec = btn(-1);

    let main_div = div();
    main_div.append(&btn_inc);
    main_div.append(&btn_dec);

    let result = div();
    result.set_id("result");
    result.set_text_content(Some("0"));
    result.set_style(Style::FontSize, "22px");

    let btns = document::Document::get().get_elements_by_tag_name("BUTTON");
    for btn in btns.iter() {
        // set their fontSize to 22 pixesl
        btn.set_style(Style::FontSize, "22px");
    }
}
```

- Build and serve using dister:

`dister build` or `dister serve`

Livid also a higher level widgets api:
```rust,no_run
use livid::{enums::*, prelude::*, *};

enum Action {
    Increment(i32),
    Decrement(i32),
}

fn btn(action: Action) -> button::Button {
    let (label, color, step) = {
        match action {
            Action::Increment(v) => ("Increment", Color::Green, v),
            Action::Decrement(v) => ("Decrement", Color::Red, -v),
        }
    };
    let btn = button::Button::default().with_label(label);
    btn.set_label_size(20);
    btn.set_label_color(color);
    btn.set_frame(FrameType::RFlatBox);
    btn.add_callback(Event::Click, move |_| {
        let frame = widget::Widget::from_id("result").unwrap();
        let mut old: i32 = frame.text_content().unwrap().parse().unwrap();
        old += step;
        frame.set_text_content(Some(&old.to_string()));
    });
    btn
}

fn main() {
    let win = window::Window::default().with_size(400, 300);
    win.set_color(Color::Custom((250, 250, 250)));
    let col = group::Column::default_fill();
    btn(Action::Increment(1));
    frame::Frame::default().with_label("0").with_id("result");
    btn(Action::Decrement(1));
    col.end();
    win.end();
}
```

## Example with CSS
```rust,no_run
use livid::{
    document::Document,
    enums::WidgetType::{self, *},
    widget::Widget,
};

fn w(typ: WidgetType) -> Widget {
    Widget::new(typ)
}

fn main() {
    Document::get().set_title("Form");
    Document::add_css_link("https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css");
    Document::add_css_link("https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css");

    let form = w(Form);
    form.set_class_name("box");
    form.append(&{
        let div = w(Div);
        div.set_class_name("field");
        div.append(&{
            let label = w(Label);
            label.set_class_name("label");
            label.set_inner_html(r#"<span class='fa fa-envelope'></span> Email"#);
            label
        });
        div.append(&{
            let div = w(Div);
            div.set_class_name("control");
            div.append(&{
                let inp = w(Input);
                inp.set_class_name("input");
                inp.set_attribute("type", "email").unwrap();
                inp.set_attribute("placeholder", "m@gmail.com").unwrap();
                inp
            });
            div
        });
        div
    });
}
```

![image](https://user-images.githubusercontent.com/37966791/161538847-9a5b564e-90a9-4555-bd9e-37946cad379f.png)

## Usage without dister:
- Install wasm-bindgen-cli:

`cargo install wasm-bindgen-cli`

- Create a project:
```toml
[project]
name = "myapp"

[dependencies]
livid = "0.1"
```

```rust,no_run
// as above
```

- Create a dist/index.html file:
```html
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    <script src="./myapp.js"></script>
    <script type="module">
      import init from "./myapp.js";
      init();
    </script>
  </body>
</html>
```

- Build your project with cargo:

`cargo build --release --target wasm32-unknown-unknown`

- Run wasm-bindgen on your generated wasm file

`wasm-bindgen target/wasm32-unknown-unknown/debug/myapp.wasm --out-dir dist --no-typescript --weak-refs`

Notice that the argument weak-refs is passed to wasm-bindgen to enable callback cleanup from the JS side.

This will generate several js glue code inside a `dist` directory, allowing the loading of your wasm binary.

- Serve your index.html using a server of your choosing:

`python3 -m http.server --dir dist`
*/

pub mod button;
pub mod console;
pub mod document;
pub mod enums;
mod error;
pub mod frame;
pub mod group;
pub mod input;
pub mod prelude;
mod traits;
pub mod utils;
pub mod widget;
pub mod window;

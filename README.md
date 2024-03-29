# livid-rs

livid-rs is a Rust framework for creating frontend and backend wasm web apps (targetting browsers and desktop).
It's composed of:
- livid:
A lightweight frontend Rust crate for creating web apps via webassembly. It provides the following advantagges:
    - Backend agnostic.
    - Thin wrapper around web-sys
    - No vdom.
    - No macros!

- livid-cli:
A lightweight build/dev tool which allows compiling your frontend into a web app or desktop app (via `livid deploy`). It can be invoked from the command line:
```
livid build
livid serve
# if targetting desktop
livid deploy 
```

- livid-desktop:
A minimal webview implementation in Rust, which allows deploying your wasm web app as desktop apps. It's also frontend agnostic.

- livid-server:
An internal server used in livid-cli and livid-desktop

- Examples:
    - Different architectures (low-level, widgets api, elm architecture).
    - Fullstack livid application
    - livid frontend with a wry backend.

## livid

## Requirements
- The wasm32-unknown-unknown target:

`rustup target add wasm32-unknown-unknown`

## Usage
- Install the livid-cli crate (to simplify building and bundling your web app):

`cargo install livid-cli`

* You can add links to CSS files/urls, and use Widget::set_class_name() to benefit from CSS styling.

- Create a project:
```toml
[dependencies]
livid = "0.2"
```

In your Rust source file:
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
    btn.set_margin(10);
    btn.set_padding(10);
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
    let col = group::Column::default_fill();
    col.set_justify_content(AlignContent::Center);
    btn(Action::Increment(1));
    let f = frame::Frame::default().with_label("0").with_id("result");
    f.set_padding(20);
    f.set_label_size(20);
    btn(Action::Decrement(1));
    col.end();
}
```

![image](https://user-images.githubusercontent.com/37966791/162965079-25fa2c36-b11f-491c-aa80-d0fc6f279839.png)

- Build and serve using livid-cli:

`livid build` or `livid serve`

- Build a desktop app with wasm for the frontend using:

`livid deploy --width=600 --height=400`

## Low-level api

Livid also a lower level widgets api:

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

## Low level example with CSS
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

## Higher-level api
Livid is unopinionated. You can build higher-level abstractions on top:
```rust,no_run
mod detail; // we define an OnEvent trait for our buttons
use crate::detail::{OnEvent, App, Settings};
use livid::{enums::*, prelude::*, *};

#[derive(Default, Copy, Clone)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl App for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - livid")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = group::Column::default_fill();
        button::Button::default()
            .with_label("Increment")
            .on_trigger(Message::IncrementPressed);
        frame::Frame::default().with_label(&self.value.to_string());
        button::Button::default()
            .with_label("Decrement")
            .on_trigger(Message::DecrementPressed);
        col.end();
    }
}

fn main() {
    Counter::new().run(Settings {
        size: (300, 200),
        win_color: Some(Color::Rgb(Rgb(250, 250, 250))),
        ..Default::default()
    })
}
```

Check the examples directory for fuller examples.

## livid-cli

livid-cli builds and bundles your wasm web app.

## Installation
`cargo install livid-cli`

## Requirements
- wasm32-unknown-unknown target:

`rustup target add wasm32-unknown-unknown`

- Wasm-bindgen:

`cargo install wasm-bindgen-cli`

- Wasm-opt (optional, wasm-opt is called if found in PATH and the --release flag is passed to `livid build`).

It can be downloaded from the [binaryen repo's releases](https://github.com/WebAssembly/binaryen/releases).

## Usage
```
USAGE:
    livid <SUBCOMMAND>

SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
    deploy    Creates a desktop app using the wasm web app for frontend
    --help    Prints this message or the help of the given subcommand(s)
```

From the manifest directory of your wasm rust applications directory, run:
`livid build` or `livid build --release`

It should generate a dist folder in the manifest directory. The generated html index file can then be served by any server or by running `livid serve`. If you prefer to use another server for development, pass the dist directory to the server:
`python3 -m http.server --dir dist`

If livid-cli finds an index.html shell in the manifest directory, it will use it and replace the `{{SCRIPT}}`  placeholder (à la emscripten shell html) with a script to load the generated js glue code.
```html
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    {{SCRIPT}}
  </body>
</html>
```

To create a desktop app:
```
livid deploy --width=800 --height=600 --title="app name"
```

## livid-desktop
# Usage
```toml
livid-desktop = "0.1"
```

In your main.rs
```rust,no_run
use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        port: 8080, // the default
        dist_folder: PathBuf::from("dist"), // the default
        ..Default::default()
    });
    a.run();
}
```

The dist folder should contain the index.html plus the wasm and javascript glue code.

## Requirements
- On Windows: No other dependencies.
- On MacOS: No other dependencies.
- On X11/wayland platforms, webkit2gtk:
    - Debian-based distros: sudo apt-get install libwebkit2gtk-4.0-dev.
    - RHEL-based distros: sudo dnf install webkit2gtk3-devel.
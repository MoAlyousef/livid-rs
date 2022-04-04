# livid

livid is a lightweight frontend Rust crate for creating web apps via webassembly. It's a thin wrapper around web-sys and also light on macros!

## Requirements
- The wasm32-unknown-unknown target:
`rustup target add wasm32-unknown-unknown`

## Usage
- Install the trunk crate (to simplify building and bundling your web app):
`cargo install trunk`

- Create an index.html file in your root directory:
```html
<html>
  <head>
    <meta charset="utf-8">
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
</html>
```
* You can add links to CSS files/urls, and use Widget::set_class_name() to benefit from CSS styling.

- Create a project:
```toml
[dependencies]
livid = "0.1"
```

In your Rust source file:

```rust
use livid::{Event, Style, Widget, WidgetType, document};

fn btn() -> Widget {
    Widget::new(WidgetType::Button)
}

fn div() -> Widget {
    Widget::new(WidgetType::Div)
}

fn increment_by(i: i32) {
    let result = document().get_element_by_id("result").unwrap();
    let mut old: i32 = result.text_content().unwrap().parse().unwrap();
    old += i;
    result.set_text_content(Some(&old.to_string()));
}

fn main() {
    document().set_title("Counter");

    let btn_inc = btn();
    btn_inc.set_text_content(Some("Increment"));
    btn_inc.set_style(Style::Color, "green");
    btn_inc.add_callback(Event::Click, move |_| increment_by(1));

    let btn_dec = btn();
    btn_dec.set_text_content(Some("Decrement"));
    btn_dec.set_style(Style::Color, "red");
    btn_dec.add_callback(Event::Click, move |_| increment_by(-1));

    let main_div = div();
    main_div.append_child(&btn_inc).unwrap();
    main_div.append_child(&btn_dec).unwrap();

    let result = div();
    result.set_id("result");
    result.set_text_content(Some("0"));
    result.set_style(Style::FontSize, "22px");

    let btns = document().get_elements_by_tag_name("BUTTON");
    for i in 0..btns.length() {
        // set their fontSize to 22 pixesl
        Widget::from(btns.item(i).unwrap()).set_style(Style::FontSize, "22px");
    }
}
```

- Build and serve using Trunk:
`trunk build` or `trunk serve`

## Example with CSS
```rust
use livid::{Document, Widget, WidgetType::{self, *}};

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
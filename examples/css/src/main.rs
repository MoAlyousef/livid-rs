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
    Document::add_css_link(
        "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css",
    );

    let form = w(Form);
    form.set_class_name("box");
    form.append(&{
        let div = w(Div);
        div.set_class_name("field");
        div.append(&{
            let label = w(Label);
            label.set_class_name("label");
            label.set_inner_html(r#"<span class='fa fa-email'></span> Email"#);
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
        div.append(&{
            let div = w(Div);
            div.set_class_name("field");
            div.append(&{
                let label = w(Label);
                label.set_text_content(Some("Password"));
                label.append(&{
                    let div = w(Div);
                    div.set_class_name("control");
                    div.append(&{
                        let inp = w(Input);
                        inp.set_class_name("input");
                        inp.set_attribute("type", "password").unwrap();
                        inp.set_attribute("placeholder", "**********").unwrap();
                        inp
                    });
                    div
                });
                label
            });
            div
        });
        div.append(&{
            let btn = w(Button);
            btn.set_class_name("button is-primary");
            btn.set_text_content(Some("Sign in"));
            btn
        });
        div
    });
}

use livid::{
    document::Document,
    enums::{Event, WidgetType::{self, *}},
    widget::Widget,
};

fn w(typ: WidgetType) -> Widget {
    Widget::new(typ)
}

fn expand_burger(_: &Widget) {
    let nav_menu = Widget::from_id("navmenu").unwrap();
    let is_active = nav_menu.class_name().contains("is-active");
    if is_active {
        nav_menu.set_class_name("navbar-menu");
    } else {
        nav_menu.set_class_name("navbar-menu is-active");
    }
}

fn create_navbar() {
    let n = w(Nav);
    n.set_class_name("navbar bd-navbar");
    n.append(&{
        let d = w(Div);
        d.set_class_name("navbar-brand");
        d.append(&{
            let a = w(A);
            a.set_class_name("navbar-item");
            // a.append(&{
            //     let i = w(Img);
            //     i.set_attribute("src", "assets/brand.png");
            //     i
            // });
            // a.add_callback();
            a
        });
        d.append(&{
            let a = w(A);
            a.set_class_name("navbar-burger");
            a.append(&{ w(Span) });
            a.append(&{ w(Span) });
            a.append(&{ w(Span) });
            a.set_attribute("data-target", "navmenu").unwrap();
            a.add_callback(Event::Click, expand_burger);
            a
        });
        d
    });
    n.append(&{
        let d = w(Div);
        d.set_id("navmenu");
        d.set_class_name("navbar-menu");
        d.append(&{
            let d = w(Div);
            d.set_class_name("navbar-start");
            d.append(&{
                let a = w(A);
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Home"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = w(A);
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Projects"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = w(A);
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Contact"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = w(A);
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Resume"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = w(A);
                a.set_class_name("navbar-item");
                a.set_text_content(Some("About"));
                // a.add_callback();
                a
            });
            d
        });
        d.append(&{
            let d = w(Div);
            d.set_class_name("navbar-end");
            d
        });
        d
    });
}

fn create_form() {
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

fn main() {
    Document::get().set_title("Form");
    Document::add_css_link("https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css");
    Document::add_css_link(
        "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css",
    );

    create_navbar();

    create_form();
}

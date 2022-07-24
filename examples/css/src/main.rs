use livid::{
    document::Document,
    enums::Event,
    widget::Widget,
    widgets::*,
};

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
    let n = nav();
    n.set_class_name("navbar bd-navbar");
    n.append(&{
        let d = div();
        d.set_class_name("navbar-brand");
        d.append(&{
            let a = a();
            a.set_class_name("navbar-item");
            // a.append(&{
            //     let i = img();
            //     i.set_attribute("src", "assets/brand.png");
            //     i
            // });
            // a.add_callback();
            a
        });
        d.append(&{
            let a = a();
            a.set_class_name("navbar-burger");
            a.append(&{ span() });
            a.append(&{ span() });
            a.append(&{ span() });
            a.set_attribute("data-target", "navmenu").unwrap();
            a.add_callback(Event::Click, expand_burger);
            a
        });
        d
    });
    n.append(&{
        let d = div();
        d.set_id("navmenu");
        d.set_class_name("navbar-menu");
        d.append(&{
            let d = div();
            d.set_class_name("navbar-start");
            d.append(&{
                let a = a();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Home"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = a();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Projects"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = a();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Contact"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = a();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Resume"));
                // a.add_callback();
                a
            });
            d.append(&{
                let a = a();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("About"));
                // a.add_callback();
                a
            });
            d
        });
        d.append(&{
            let d = div();
            d.set_class_name("navbar-end");
            d
        });
        d
    });
}

fn create_form() {
    let form = form();
    form.set_class_name("box");
    form.append(&{
        let d = div();
        d.set_class_name("field");
        d.append(&{
            let label = label();
            label.set_class_name("label");
            label.set_inner_html(r#"<span class='fa fa-email'></span> Email"#);
            label
        });
        d.append(&{
            let d = div();
            d.set_class_name("control");
            d.append(&{
                let inp = input();
                inp.set_class_name("input");
                inp.set_attribute("type", "email").unwrap();
                inp.set_attribute("placeholder", "m@gmail.com").unwrap();
                inp
            });
            d
        });
        d.append(&{
            let d = div();
            d.set_class_name("field");
            d.append(&{
                let label = label();
                label.set_text_content(Some("Password"));
                label.append(&{
                    let d = div();
                    d.set_class_name("control");
                    d.append(&{
                        let inp = input();
                        inp.set_class_name("input");
                        inp.set_attribute("type", "password").unwrap();
                        inp.set_attribute("placeholder", "**********").unwrap();
                        inp
                    });
                    d
                });
                label
            });
            d
        });
        d.append(&{
            let btn = button();
            btn.set_class_name("button is-primary");
            btn.set_text_content(Some("Sign in"));
            btn
        });
        d
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

use livid::{prelude::*, widget::Widget, enums::*};

fn div() -> Widget {
    Widget::new(WidgetType::Div)
}

fn btn(i: i32) -> Widget {
    let btn = Widget::new(WidgetType::Button);
    let (label, col) = if i > 0 {
        ("Increment", "green")
    } else {
        ("Decrement", "red")
    };
    btn.set_text_content(Some(label));
    btn.set_style(Style::Color, col);
    btn.add_callback(Event::Click, move |_| {
        let result = Widget::from_id("result").unwrap();
        let mut old: i32 = result.text_content().unwrap().parse().unwrap();
        old += i;
        result.set_text_content(Some(&old.to_string()));
    });
    btn
}

fn main() {
    Document::get().set_title("Counter");

    let btn_inc = btn(1);
    let btn_dec = btn(-1);

    let main_div = div();
    main_div.append(&btn_inc);
    main_div.append(&btn_dec);

    let result = div();
    result.set_id("result");
    result.set_text_content(Some("0"));
    result.set_style(Style::FontSize, "22px");

    let btns = Document::get().get_elements_by_tag_name("BUTTON");
    for btn in btns.iter() {
        // set their fontSize to 22 pixesl
        btn.set_style(Style::FontSize, "22px");
    }
}
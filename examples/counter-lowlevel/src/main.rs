use livid::{enums::*, prelude::*, *, widgets::*};

fn btn(i: i32) -> widget::Widget {
    // this is akin to calling `widget::Widget::new(WidgetType::Button)`
    let btn = button();
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
    let main_div = div();
    main_div.begin();
    btn(1);
    btn(-1);
    main_div.end();

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

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
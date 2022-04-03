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

    let btn_dec = Widget::new(WidgetType::Button);
    btn_dec.set_text_content(Some("Decrement"));
    btn_dec.set_style(Style::Color, "red");
    btn_dec.add_callback(Event::Click, {
        let cnt = count.clone();
        move |_| {
            *cnt.borrow_mut() -= 1;
            let result = document().get_element_by_id("result").unwrap();
            result.set_text_content(Some(&cnt.borrow().to_string()));
    }});

    let div = Widget::new(WidgetType::Div);
    div.append_child(&btn_inc).unwrap();
    div.append_child(&btn_dec).unwrap();

    let result = Widget::new(WidgetType::Div);
    result.set_id("result");
    result.set_text_content(Some("0"));
    result.set_style(Style::FontSize, "22px");

    let btns = document().get_elements_by_tag_name("BUTTON");
    for i in 0..btns.length() {
        // set their fontSize to 22 pixesl
        Widget::from(btns.item(i).unwrap()).set_style(Style::FontSize, "22px");
    }
}
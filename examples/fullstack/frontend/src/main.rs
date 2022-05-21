use livid::{enums::*, prelude::*, *};

fn main() {
    let inp = input::Input::default();
    let btn1 = button::Button::default().with_label("add two");
    let btn2 = button::Button::default().with_label("add three");
    let btn3 = button::Button::default().with_label("Get global val");
    let quit_btn = button::Button::default().with_label("quit");
    frame::Frame::default().with_id("result");

    btn1.add_callback(Event::Click, {
        let inp = inp.clone();
        move |_| {
        rt::call("addTwo", &[&inp.value()]).ok();
    }});

    // Another way of doing things!
    btn2.add_callback(Event::Click, move |_| {
        rt::call("call", &[&format!("addThree:{}",inp.value())]).ok();
    });

    btn3.add_callback(Event::Click, move |_| {
        let val = rt::get_variable("globalVal").unwrap();
        rt::alert(&format!("{}", val.as_f64().unwrap()));
    });

    quit_btn.add_callback(Event::Click, move |_| {
        rt::call("quit", &[]).ok();
    });
}
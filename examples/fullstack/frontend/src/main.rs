use livid::{enums::*, prelude::*, *};

fn main() {
    window::Window::default_fill();
    let inp = input::Input::default();
    let btn = button::Button::default().with_label("add two");
    frame::Frame::default().with_id("result");

    btn.add_callback(Event::Click, move |_| {
        rt::call("addTwo", &[&inp.value()]);
    });
}
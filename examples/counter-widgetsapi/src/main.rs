use livid::{enums::*, prelude::*, *};

const RED: Color = Color(255, 0, 0);
const GREEN: Color = Color(0, 255, 0);

enum Action {
    Increment(i32),
    Decrement(i32),
}

fn btn(action: Action) -> button::Button {
    let (label, color, step) = {
        match action {
            Action::Increment(v) => ("Increment", GREEN, v),
            Action::Decrement(v) => ("Decrement", RED, v * -1),
        }
    };
    let btn = button::Button::default().with_label(label);
    btn.set_label_color(color);
    btn.add_callback(Event::Click, move |_| {
        let frame = widget::Widget::from_id("result").unwrap();
        let mut old: i32 = frame.text_content().unwrap().parse().unwrap();
        old += step;
        frame.set_text_content(Some(&old.to_string()));
    });
    btn
}

fn main() {
    let win = window::Window::default().with_size(400, 300);
    win.set_color(Color(250, 250, 250));
    let col = group::Column::default();
    btn(Action::Increment(1));
    frame::Frame::default().with_label("0").with_id("result");
    btn(Action::Decrement(1));
    col.end();
    win.end();
}

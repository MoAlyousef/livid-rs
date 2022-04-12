use livid::{enums::*, prelude::*, *};

fn main() {
    let win = window::Window::default().with_size(400, 300);
    win.set_color(Color::Custom((250, 250, 250)));
    win.end();
}

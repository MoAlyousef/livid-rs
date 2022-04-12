use livid::{enums::*, prelude::*, *};

fn main() {
    let win = window::Window::default().with_size(400, 300);
    win.set_color(Color::Custom((250, 250, 250)));
    let choice = menu::NavBar::default();
    choice.set_color(Color::Green);
    let link = choice.add_choice("123");
    link.set_href("#here");
    choice.add_choice("321");
    win.end();
}

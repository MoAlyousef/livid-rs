use livid::{enums::*, prelude::*, *};

fn main() {
    let win = window::Window::default_fill();
    win.set_color(Color::Rgb(Rgb(240, 240, 240)));
    let choice = menu::MenuBar::default();
    choice.set_color(Color::LightBlue);
    let link = choice.add_choice("Home");
    link.set_href("#here");
    let link = choice.add_choice("News");
    link.set_href("#here");
    let link = choice.add_choice("About");
    link.set_href("#here");
    let link = choice.add_choice("Overthere");
    link.set_href("#here");
    link.add_callback(Event::MouseOver, |l| l.set_color(Color::Red));
    choice.add_choice("321");

    let choice = menu::NavBar::default();
    choice.set_color(Color::LightBlue);
    let link = choice.add_choice("Home");
    link.set_href("#here");
    let link = choice.add_choice("News");
    link.set_href("#here");
    let link = choice.add_choice("About");
    link.set_href("#here");
    let link = choice.add_choice("Overthere");
    link.set_href("#here");
    link.add_callback(Event::MouseOver, |l| l.set_color(Color::Red));
    choice.add_choice("321");
    win.end();
}

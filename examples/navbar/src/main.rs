use livid::{enums::*, prelude::*, *};

fn main() {
    let table_data = &[
        vec!["1", "2", "3", "4"],
        vec!["name", "age", "job", "city"]
    ];

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

    // let choice = menu::NavBar::default();
    // choice.set_color(Color::LightBlue);
    // let link = choice.add_choice("Home");
    // link.set_href("#here");
    // let link = choice.add_choice("News");
    // link.set_href("#here");
    // let link = choice.add_choice("About");
    // link.set_href("#here");
    // let link = choice.add_choice("Overthere");
    // link.set_href("#here");
    // link.add_callback(Event::MouseOver, |l| l.set_color(Color::Red));
    // choice.add_choice("321");

    let table = table::TableView::default();
    table.set_view(table_data);
    win.end();
}

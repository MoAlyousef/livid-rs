use livid::{enums::*, prelude::*, *};

fn create_table() -> table::TableView {
    let table_data = &[
        vec!["1", "2", "3", "4"],
        vec!["name", "age", "job", "city"]
    ];
    let table = table::TableView::default();
    table.set_view(table_data);
    table
}

fn main() {
    // document::Document::add_css_link("https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css");
    let win = window::Window::default_fill();
    win.set_color(Color::Rgb(Rgb(240, 240, 240)));
    let choice = menu::MenuBar::default();
    let link = choice.add_choice("Home");
    link.set_href("#here");
    let link = choice.add_choice("News");
    link.set_href("#here");
    let link = choice.add_choice("About");
    link.set_href("#here");
    let link = choice.add_choice("Overthere");
    link.set_href("#here");
    link.add_callback(Event::MouseOver, |l| l.set_color(Color::LightBlue));
    choice.add_choice("321");

    let row = group::Row::default();
    let choice = menu::NavBar::default();
    let link = choice.add_choice("Home");
    link.set_href("#here");
    let link = choice.add_choice("News");
    link.set_href("#here");
    let link = choice.add_choice("About");
    link.set_href("#here");
    let link = choice.add_choice("Overthere");
    link.set_href("#here");
    link.add_callback(Event::MouseOver, |l| l.set_color(Color::LightBlue));
    choice.add_choice("321");
    
    create_table();
    row.end();

    win.end();
}

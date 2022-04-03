use di::{Widget, WidgetType, Event, Style};

fn main() {
    let div = Widget::new(WidgetType::Div);
    div.set_text_content(Some("Hello"));
    div.set_callback(Event::Click, |d| d.set_text_content(Some("Clicked")));
    div.set_style(Style::BackgroundColor, "red");
}


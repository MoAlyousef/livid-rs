use livid::{widget::Widget, enums::*};

fn ul() -> Widget {
    Widget::new(WidgetType::Ul)
}

fn li() -> Widget {
    Widget::new(WidgetType::Li)
}

fn svg() -> Widget {
    Widget::new(WidgetType::Svg)
}

fn div() -> Widget {
    Widget::new(WidgetType::Div)
}

fn main() {
    let ul = ul();
    for i in 0..10 {
        let li = li();
        li.set_text_content(Some(&i.to_string()));
        ul.append(&li);
    }

    let svg = svg();
    svg.set_attribute("viewBox", "0 0 300 300").unwrap();
    svg.set_attribute("width", "300").unwrap();
    svg.set_attribute("height", "300").unwrap();
    svg.set_style(Style::Display, "block");
    svg.set_inner_html(
        r#"
        <circle cx="50" cy="50" r="40" stroke="black" stroke-width="3" fill="red"></circle>
    "#,
    );

    let div = div();
    div.set_outer_html(
        r#"
        <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
          <circle cx="50" cy="50" r="50"/>
        </svg>    
        "#,
    );
}

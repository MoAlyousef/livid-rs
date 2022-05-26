mod my_widget;
use my_widget::MyMenuBar;
use livid::{enums::*, prelude::*, *};

fn main() {
    let choice = MyMenuBar::default();
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
    let canvas = misc::Canvas::default();
    canvas.draw(|_canvas, context| {
        use std::f64;
        context.begin_path();
    
        // Draw the outer circle.
        context
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
    
        // Draw the mouth.
        context.move_to(110.0, 75.0);
        context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();
    
        // Draw the left eye.
        context.move_to(65.0, 65.0);
        context
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
    
        // Draw the right eye.
        context.move_to(95.0, 65.0);
        context
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();
    
        context.stroke();
    });
}

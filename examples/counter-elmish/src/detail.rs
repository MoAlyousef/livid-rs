use livid::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

pub trait OnEvent<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn on_trigger(self, msg: T) -> Self
    where
        Self: Sized;
}

impl<T> OnEvent<T> for button::Button
where
    T: Send + Sync + Clone + 'static,
{
    fn on_trigger(self, msg: T) -> Self {
        let (s, _) = document::channel::<T>();
        self.add_callback(Event::Click, move |_| {
            s.send(msg.clone());
        });
        self
    }
}

#[derive(Default)]
pub struct Settings {
    pub size: (i32, i32),
    pub win_color: Option<enums::Color>,
}

pub trait App: Clone {
    type Message: Clone + Send + Sync + 'static;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self);
    fn update(&mut self, message: Self::Message);
    fn run(&mut self, settings: Settings)
    where
        Self: 'static,
    {
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let win = window::Window::default()
            .with_size(w, h)
            .with_label(&self.title());
        if let Some(color) = settings.win_color {
            win.set_color(color);
        }
        self.view();
        win.end();
        let (_, r) = document::channel::<Self::Message>();
        let s = Rc::from(RefCell::from(self.clone()));
        utils::set_interval(30, move || {
            if let Some(msg) = r.recv() {
                let mut s = s.borrow_mut();
                s.update(msg);
                win.inner().set_inner_html("");
                win.begin();
                s.view();
                win.end();
                win.redraw();
            }
        });
    }
}

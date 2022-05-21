use livid::{enums::*, prelude::*, *};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

type Chan = (
    crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>,
    crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>,
);

lazy_static::lazy_static! {
    static ref CHANNEL: Chan = crossbeam_channel::unbounded();
}

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
        self.add_callback(Event::Click, move |_| {
            CHANNEL.0.try_send(Box::new(msg.clone())).unwrap();
        });
        self
    }
}

pub struct Settings {
    pub size: (i32, i32),
    pub win_color: Option<enums::Color>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            size: (400, 300),
            win_color: Some(Color::Rgb(Rgb(250, 250, 250))),
        }
    }
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
        let win = group::Group::default().with_size(w, h);
        document::Document::get().set_title(&self.title());
        if let Some(color) = settings.win_color {
            win.set_color(color);
        }
        self.view();
        win.end();
        let r = &CHANNEL.1;
        let s = Rc::from(RefCell::from(self.clone()));
        utils::set_interval(30, move || {
            if let Ok(msg) = r.try_recv() {
                if let Ok(msg) = msg.downcast::<Self::Message>() {
                    let mut s = s.borrow_mut();
                    s.update(*msg);
                    win.clear();
                    win.begin();
                    s.view();
                    win.end();
                }
            }
        });
    }
}

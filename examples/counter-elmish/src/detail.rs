use livid::{enums::*, prelude::*, *};
use std::any::Any;
use std::cell::RefCell;
use std::marker;
use std::rc::Rc;

lazy_static::lazy_static! {
    static ref CHANNEL: (crossbeam_channel::Sender<Box<dyn Any + Send + Sync>>, crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>>) = crossbeam_channel::unbounded();
    static ref SENDER: crossbeam_channel::Sender<Box<dyn Any + Send + Sync>> = CHANNEL.clone().0;
    static ref RECEIVER: crossbeam_channel::Receiver<Box<dyn Any + Send + Sync>> = CHANNEL.clone().1;
}

/// Creates a sender struct
#[derive(Debug, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Sender<T> {}
unsafe impl<T: Send + Sync> Sync for Sender<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        SENDER.try_send(Box::new(val)).ok();
    }
}

/// Creates a receiver struct
#[derive(Debug, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
}

unsafe impl<T: Send + Sync> Send for Receiver<T> {}
unsafe impl<T: Send + Sync> Sync for Receiver<T> {}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            data: marker::PhantomData,
        }
    }
}

impl<T: 'static + Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        // if let Some(r) = &*RECEIVER {
        if let Ok(msg) = RECEIVER.try_recv() {
            if let Ok(message) = msg.downcast() {
                Some(*message)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Creates a channel returning a Sender and Receiver structs (mpsc: multiple producer single consumer).
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
    let s = Sender {
        data: marker::PhantomData,
    };
    let r = Receiver {
        data: marker::PhantomData,
    };
    (s, r)
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
        let (s, _) = channel::<T>();
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
        let (_, r) = channel::<Self::Message>();
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

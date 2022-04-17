use wasm_bindgen::JsCast;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{any, marker, mem, os::raw};

static mut SENDER: Option<crossbeam_channel::Sender<*mut raw::c_void>> = None;
static mut RECEIVER: Option<crossbeam_channel::Receiver<*mut raw::c_void>> = None;

#[repr(C)]
struct Message<T: Send + Sync> {
    hash: u64,
    sz: usize,
    msg: T,
}

/// Creates a sender struct
#[derive(Debug, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            data: marker::PhantomData,
            hash: self.hash,
            sz: self.sz,
        }
    }
}

impl<T: Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        let msg = Message {
            hash: self.hash,
            sz: self.sz,
            msg: val,
        };
        unsafe {
            if let Some(s) = &SENDER {
                s.try_send(Box::into_raw(Box::from(msg)) as *mut raw::c_void)
                    .ok();
            }
        }
    }
}

/// Creates a receiver struct
#[derive(Debug, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}
//
// Manually create the impl so there's no Clone bound on T
impl<T: Send + Sync> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver {
            data: marker::PhantomData,
            hash: self.hash,
            sz: self.sz,
        }
    }
}

impl<T: Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        if let Some(r) = unsafe { &RECEIVER } {
            if let Ok(msg) = r.try_recv() {
                if !msg.is_null() {
                    let data = unsafe { Box::from_raw(msg as *const _ as *mut Message<T>) };
                    if data.sz == self.sz && data.hash == self.hash {
                        Some(data.msg)
                    } else {
                        None
                    }
                } else {
                    None
                }
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
    unsafe {
        if SENDER.is_none() || RECEIVER.is_none() {
            let (s, r) = crossbeam_channel::unbounded();
            SENDER = Some(s);
            RECEIVER = Some(r);
        }
    }
    let msg_sz = mem::size_of::<T>();
    let type_name = any::type_name::<T>();
    let mut hasher = DefaultHasher::new();
    type_name.hash(&mut hasher);
    let type_hash = hasher.finish();

    let s = Sender {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    let r = Receiver {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    (s, r)
}

/// Get the global window
fn window() -> web_sys::Window {
    web_sys::window().expect("No global window found!")
}

/// Get the global document
fn document() -> web_sys::Document {
    window().document().expect("No document found!")
}

#[derive(Clone, Copy)]
pub struct Document;

impl Document {
    /// Get the global document
    pub fn get() -> web_sys::Document {
        document()
    }

    /// Get the global docuement's body
    pub fn body() -> web_sys::HtmlElement {
        Self::get().body().expect("No body found!")
    }

    /// Get the head
    pub fn head() -> web_sys::HtmlHeadElement {
        Self::get().head().expect("No head element found!")
    }

    /// add a link
    pub fn add_css_link(href: &str) {
        let link: web_sys::HtmlLinkElement =
            Document::get().create_element("link").unwrap().dyn_into().unwrap();
        link.set_rel("stylesheet");
        link.set_type("text/css");
        link.set_href(href);
        Self::head().append_child(&link).unwrap();
    }

    /// Check if the document has a main livid window
    pub fn has_window() -> bool {
        crate::window::HAS_WINDOW.load(std::sync::atomic::Ordering::Relaxed)
    }
}

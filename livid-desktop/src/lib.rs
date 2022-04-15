/*!
# livid-desktop

Lightly wraps a native webview which can be used with livid for the frontend.

# Usage
```toml
livid-desktop = "0.1"
```

In your main.rs
```rust,no_run
use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        port: 8080, // the default
        dist_folder: PathBuf::from("dist"), // the default
        ..Default::default()
    });
    a.run();
}
```

The dist folder should contain the index.html plus the wasm and javascript glue code.

## Requirements
- On Windows: No other dependencies.
- On MacOS: No other dependencies.
- On X11/wayland platforms, webkit2gtk:
    - Debian-based distros: sudo apt-get install libwebkit2gtk-4.0-dev.
    - RHEL-based distros: sudo dnf install webkit2gtk3-devel.
*/

use fltk_webview_sys as wv;
use livid_server::Server;
use std::path::PathBuf;
use wv::webview_t;

pub struct Settings {
    pub w: i32,
    pub h: i32,
    pub title: &'static str,
    pub port: &'static str,
    pub fixed: bool,
    pub debug: bool,
    pub dist_folder: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            w: 600,
            h: 400,
            title: "app",
            port: "8080",
            fixed: true,
            debug: false,
            dist_folder: PathBuf::from("dist"),
        }
    }
}

#[cfg(target_os = "macos")]
extern "C" {
    fn add_nsmenu(val: bool);
}

pub struct App {
    wv: webview_t,
    settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        let wv = unsafe {
            let wv = wv::webview_create(settings.debug as i32, std::ptr::null_mut() as _);
            let title = std::ffi::CString::new(settings.title).unwrap();
            wv::webview_set_title(wv, title.as_ptr() as _);
            wv::webview_set_size(
                wv,
                settings.w,
                settings.h,
                if settings.fixed { 3 } else { 0 },
            );
            #[cfg(target_os = "macos")]
            add_nsmenu();
            wv
        };
        Self { wv, settings }
    }
    pub fn run(self) {
        let port = self.settings.port;
        let dist_folder = self.settings.dist_folder;
        std::thread::spawn(move || {
            Server::serve(port, &std::env::current_dir().unwrap().join(dist_folder))
        });
        let addr = format!("http://127.0.0.1:{}", port);
        let addr = std::ffi::CString::new(addr).unwrap();
        unsafe {
            wv::webview_navigate(self.wv, addr.as_ptr() as _);
            wv::webview_run(self.wv);
        }
    }
}

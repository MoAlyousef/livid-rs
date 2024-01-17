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
use std::path::PathBuf;

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        port: "8080", // the default
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
#![allow(clippy::needless_doctest_main)]

use wv_sys::*;
use livid_server::Server;
use std::path::PathBuf;

pub struct Settings {
    pub w: i32,
    pub h: i32,
    pub title: &'static str,
    pub port: u16,
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
            port: 8080,
            fixed: true,
            debug: false,
            dist_folder: PathBuf::from("dist"),
        }
    }
}

pub struct App {
    wv: Webview,
    settings: Settings,
}

impl App {
    /// Create a new app
    pub fn new(settings: Settings) -> Self {
        let mut wv = Webview::create_no_win(settings.debug);
        wv.set_title(settings.title);
        wv.set_size(
            settings.w,
            settings.h,
            if settings.fixed {
                SizeHint::Fixed
            } else {
                SizeHint::None
            },
        );
        Self { wv, settings }
    }
    /// Get the webview of the app
    pub fn get_webview(&self) -> Webview {
        self.wv.clone()
    }
    /// Run the app
    pub fn run(mut self) {
        let port = self.settings.port;
        let dist_folder = self.settings.dist_folder;
        std::thread::spawn(move || {
            let mut server = Server::new(port);
            server.serve_dir(&std::env::current_dir().unwrap().join(dist_folder));
            server.serve();
        });
        let addr = format!("http://127.0.0.1:{}", port);
        self.wv.navigate(&addr);
        self.wv.run();
    }
}

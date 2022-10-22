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

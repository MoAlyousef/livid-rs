use livid_desktop::{App, Settings};
use std::path::PathBuf;

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        dist_folder: PathBuf::from("examples/simple_js_dist"),
        ..Default::default()
    });
    a.run();
}
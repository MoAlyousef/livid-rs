use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        ..Default::default()
    });
    a.run();
}
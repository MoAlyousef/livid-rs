use livid_desktop::{App, Settings};
use std::path::PathBuf;

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: true,
        dist_folder: PathBuf::from("examples/dispatch_dist"),
        ..Default::default()
    });
    let mut wv = a.get_webview();
    wv.bind("my_dispatch", {
        let mut wv = wv.clone();
        move |_, content| {
            let content = content[2..content.len() - 2].to_string();
            // dispatch from the main thread
            wv.dispatch(move |_| {
                // spawn another thread to not block our main thread while waiting
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    println!("{}", content);
                });
            });
        }
    });
    a.run();
}

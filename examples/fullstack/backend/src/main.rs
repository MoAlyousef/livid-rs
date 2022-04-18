use livid_desktop::{App, Settings};
use tinyjson::JsonValue;

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        port: "8000",
        fixed: true,
        ..Default::default()
    });

    let mut wv = a.get_webview();

    wv.init("window.globalVal = 2;");

    wv.bind("quit", {
        |_, _| {
            std::process::exit(0);
        }
    });

    wv.bind("addTwo", {
        let mut wv = wv.clone();
        move |_, content| {
            if let Ok(JsonValue::Array(args)) = content.parse() {
                if let JsonValue::String(s) = &args[0] {
                    if let Ok(val) = s.parse::<f64>() {
                        let ret = val + 2.0;
                        wv.eval(&format!(
                            "window.globalVal = {0}; document.getElementById('result').innerText = {0};",
                            ret
                        ));
                    }
                }
            }
        }
    });

    // Anothre way of doing things
    wv.bind("call", {
        let mut wv = wv.clone();
        move |_, content| {
            // strip first and last angle brackets and quotes
            let c = &content[2..content.len() - 2];
            if let Some(val) = c.strip_prefix("addThree:") {
                if let Ok(val) = val.parse::<f64>() {
                    let ret = val + 3.0;
                    wv.eval(&format!(
                        "window.globalVal = {0}; document.getElementById('result').innerText = {0};",
                        ret
                    ));
                }
            }
        }
    });

    a.run();
}

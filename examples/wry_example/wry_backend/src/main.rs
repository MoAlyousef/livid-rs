use std::fs::{canonicalize, read};
use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    http::ResponseBuilder,
    webview::WebViewBuilder,
};

enum UserEvents {
    AddTwo(f64),
  }

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::<UserEvents>::with_user_event();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("My App")
        .build(&event_loop)
        .unwrap();

    let handler = move |_window: &Window, req: String| {
        if req == "quit" {
            std::process::exit(0);
        }
        if let Some(val) = req.strip_prefix("addTwo:") {
            if let Ok(val) = val.parse::<f64>() {
                let _ = proxy.send_event(UserEvents::AddTwo(val));
            }
        }
    };

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_custom_protocol("wry".into(), move |request| {
            let path = request.uri().replace("wry://", "");
            let content = read(canonicalize(&path)?)?;
            let (data, meta) = if path.ends_with(".html") {
                (content, "text/html")
            } else if path.ends_with(".js") {
                (content, "application/javascript")
            } else if path.ends_with(".wasm") {
                (content, "application/wasm")
            } else {
                unimplemented!();
            };

            ResponseBuilder::new().mimetype(meta).body(data)
        })
        .with_initialization_script("window.globalVal = 2;")
        .with_ipc_handler(handler)
        .with_url("wry://dist/index.html")?
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(UserEvents::AddTwo(val)) => {
                webview.evaluate_script(&format!(
                    "window.globalVal = {0}; document.getElementById('result').innerText = {0};",
                    val + 2.0
                )).unwrap();
              }
            _ => (),
        }
    });
}

use ascii::AsciiString;
use std::fs;
use std::path::Path;

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        "js" => "application/javascript",
        "wasm" => "application/wasm",
        _ => "text/plain; charset=utf8",
    }
}

pub struct Server;

impl Server {
    pub fn serve(port: &str, root: &Path) {
        let server = tiny_http::Server::http(&format!("0.0.0.0:{}", port)).unwrap();
        loop {
            let rq = match server.recv() {
                Ok(rq) => rq,
                Err(_) => break,
            };

            let mut url = rq.url().to_string().strip_prefix('/').unwrap().to_string();
            if url.is_empty() {
                url = "index.html".to_string();
            }
            let path = Path::new(&url);
            let npath = root.join(path);
            let file = fs::File::open(&npath);

            if file.is_ok() {
                let response = tiny_http::Response::from_file(file.unwrap());

                let response = response.with_header(tiny_http::Header {
                    field: "Content-Type".parse().unwrap(),
                    value: AsciiString::from_ascii(get_content_type(&path)).unwrap(),
                });

                let _ = rq.respond(response);
            } else {
                let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
                let _ = rq.respond(rep);
            }
        }
    }
}
use ascii::AsciiString;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Method, Request, Response, ResponseBox, Server as ThServer, StatusCode};

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
        "css" => "text/css",
        "wasm" => "application/wasm",
        _ => "text/plain; charset=utf8",
    }
}

type Routes = HashMap<(Method, String), fn(rq: &mut Request) -> ResponseBox>;

pub struct Server {
    port: u16,
    root: PathBuf,
    static_serve: bool,
    routes: Routes,
}

impl Server {
    pub fn new<P: AsRef<Path>>(port: u16, root: &P) -> Self {
        Self {
            port,
            root: PathBuf::from(root.as_ref()),
            static_serve: false,
            routes: HashMap::new(),
        }
    }
    pub fn route(&mut self, verb: Method, url: &str, f: fn(rq: &mut Request) -> ResponseBox) {
        self.routes.insert((verb, url.to_string()), f);
    }
    pub fn static_serve(&mut self, flag: bool) {
        self.static_serve = flag;
    }
    pub fn serve(&mut self) {
        let server = ThServer::http(&format!("0.0.0.0:{}", self.port)).unwrap();
        while let Ok(mut rq) = server.recv() {
            let method = rq.method().clone();
            let url = rq.url().to_string();
            if let Some(f) = self.routes.get(&(method.clone(), url)) {
                let resp = f(&mut rq);
                let _ = rq.respond(resp);
            } else if self.static_serve && method == Method::Get {
                let mut url = rq.url().to_string().strip_prefix('/').unwrap().to_string();
                if url.is_empty() {
                    url = "index.html".to_string();
                }
                let path = Path::new(&url);
                let npath = self.root.join(path);
                let file = fs::File::open(&npath);
                if let Ok(file) = file {
                    let response = Response::from_file(file);
                    let response = response.with_header(Header {
                        field: "Content-Type".parse().unwrap(),
                        value: AsciiString::from_ascii(get_content_type(path)).unwrap(),
                    });
                    let _ = rq.respond(response);
                } else {
                    let rep = Response::new_empty(StatusCode(404));
                    let _ = rq.respond(rep);
                }
            }
        }
    }
}

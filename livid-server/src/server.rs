use ascii::AsciiString;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
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
    root: Option<PathBuf>,
    output: Option<Box<dyn Write>>,
    routes: Routes,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            root: None,
            output: None,
            routes: HashMap::new(),
        }
    }
    pub fn serve_dir<P: AsRef<Path>>(&mut self, dir: &P) {
        self.root = Some(PathBuf::from(dir.as_ref()));
    }
    pub fn log_output(&mut self, o: Box<dyn std::io::Write>) {
        self.output = Some(o);
    }
    pub fn route(&mut self, verb: Method, url: &str, f: fn(rq: &mut Request) -> ResponseBox) {
        self.routes.insert((verb, url.to_string()), f);
    }
    pub fn serve(&mut self) {
        let server = ThServer::http(&format!("0.0.0.0:{}", self.port)).unwrap();
        while let Ok(mut rq) = server.recv() {
            if let Some(output) = self.output.as_mut() {
                let v = rq.http_version();
                let ra = rq.remote_addr().unwrap();
                output.write_all(format!("{:?} - [{}] - \"{} {} HTTP/{}.{}\"", ra.ip(), httpdate::HttpDate::from(std::time::SystemTime::now()), rq.method(), rq.url(), v.0, v.1).as_bytes()).unwrap();
            }
            let method = rq.method().clone();
            let url = rq.url().to_string();
            if let Some(f) = self.routes.get(&(method.clone(), url)) {
                let resp = f(&mut rq);
                let _ = rq.respond(resp);
            } else if self.root.is_some() && method == Method::Get {
                let mut url = rq.url().to_string().strip_prefix('/').unwrap().to_string();
                if url.is_empty() {
                    url = "index.html".to_string();
                }
                let path = Path::new(&url);
                let npath = self.root.as_ref().unwrap().join(path);
                let file = fs::File::open(&npath);
                let code = if let Ok(file) = file {
                    let response = Response::from_file(file);
                    let response = response.with_header(Header {
                        field: "Content-Type".parse().unwrap(),
                        value: AsciiString::from_ascii(get_content_type(path)).unwrap(),
                    });
                    let code = response.status_code();
                    let _ = rq.respond(response);
                    code
                } else {
                    let response = Response::new_empty(StatusCode(404));
                    let code = response.status_code();
                    let _ = rq.respond(response);
                    code
                };
                if let Some(output) = self.output.as_mut() {
                    output.write_all(&format!(" - {}\n", code.0).as_bytes()).unwrap();
                    output.flush().unwrap();
                }
            }
        }
    }
}

use livid_server::Server;
use std::path::PathBuf;
use std::process::Command;

const USAGE: &str = r#"{{BIN}} {{VERSION}}
Builds and bundles your wasm web app.
USAGE:
    {{BIN}} <SUBCOMMAND>
SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
    deploy    Creates a desktop app using the wasm web app for frontend
    --help    Prints this message
"#;

const DEPLOY: &str = r#"USAGE:
    {{BIN}} deploy <OPTIONS>
OPTIONS:
    --width   Sets the window's width
    --height  Sets the window's height
    --title   Sets the window's title
    --port    Sets the server's local port
    --using   Sets the project to be used for the backend
    --help    Prints this message
"#;

const SCRIPT: &str = r#"
<script src="./{{crate}}.js"></script>
<script type="module">
  import init from "./{{crate}}.js";
  init();
</script>
"#;

const HTML: &str = r#"
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    {{SCRIPT}}
  </body>
</html>
"#;

const APP: &str = r#"use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: {{width}},
        h: {{height}},
        title: {{title}},
        port: {{port}},
        ..Default::default()
    });
    a.run();
}"#;

const CARGO: &str = r#"[package]
name = "{{crate}}"
version = "0.1.0"
edition = "2021"

[dependencies]
livid-desktop = "0.1"

[profile.release]
opt-level = 3
strip = true
"#;

pub fn handle_args(bin_name: &str, args: &[String]) {
    if args.len() == 1 {
        help(bin_name);
        return;
    }
    match args[1].as_str() {
        "build" => build(args),
        "serve" => serve(args),
        "deploy" => deploy(bin_name, args),
        "clean" => clean(),
        "--help" | "--version" => help(bin_name),
        _ => help(bin_name),
    }
}

fn check_prog(prog: &str) -> bool {
    let mut cmd = Command::new(prog);
    cmd.args(["--help"]);
    cmd.output().is_ok()
}

fn help(bin_name: &str) {
    let usage = USAGE
        .replace("{{BIN}}", bin_name)
        .replace("{{VERSION}}", env!("CARGO_PKG_VERSION"));
    println!("{}", usage);
}

fn clean() {
    let mut cargo = Command::new("cargo");
    cargo.args(["clean"]);
    cargo.spawn().unwrap().wait().unwrap();

    let dist = PathBuf::from("dist");
    if dist.exists() {
        std::fs::remove_dir_all(dist).unwrap();
    }
}

fn serve(args: &[String]) {
    build(args);
    println!("Livid server running on http://0.0.0.0:8080!\nServing dist/");
    let mut server = Server::new(8080);
    server.serve_dir(&std::env::current_dir().unwrap().join("dist"));
    server.serve();
}

fn build(args: &[String]) {
    let mut release = false;
    if let Some(val) = args.get(2) {
        if val == "--release" {
            release = true;
        }
    }
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to find a Cargo.toml!");
    let pkg: toml::Value = cargo_toml.parse().unwrap();
    let crate_name = format!("{}", pkg["package"]["name"]).replace('"', "");
    let mut path = PathBuf::from("target").join("wasm32-unknown-unknown");
    if release {
        path = path.join("release");
    } else {
        path = path.join("debug");
    }
    path = path.join(format!("{}.wasm", &crate_name));
    if !check_prog("wasm-bindgen") {
        eprintln!("wasm-bindgen-cli was not found, running a first-time install...");
        let mut cargo = Command::new("cargo");
        cargo.args(["install", "wasm-bindgen-cli"]);
        cargo.spawn().unwrap().wait().unwrap();
    }
    let mut cargo = std::process::Command::new("cargo");
    let mut cargo_args = vec!["build", "--target", "wasm32-unknown-unknown"];
    if release {
        cargo_args.push("--release");
    }
    cargo.args(&cargo_args);
    cargo.spawn().unwrap().wait().unwrap();
    if check_prog("wasm-opt") && release {
        let mut opt = Command::new("wasm-opt");
        let path = format!("{}", path.display());
        opt.args([&path, "-O3", "-o", &path]);
        opt.spawn().unwrap().wait().unwrap();
    }
    let mut wb = Command::new("wasm-bindgen");
    wb.args([
        &format!("{}", path.display()),
        "--out-dir",
        "dist",
        "--target",
        "web",
        "--weak-refs",
        "--no-typescript",
    ]);
    wb.spawn().unwrap().wait().unwrap();
    let script = SCRIPT.to_string().replace("{{crate}}", &crate_name);
    let html = std::fs::read_to_string("index.html")
        .unwrap_or_else(|_| HTML.to_string())
        .replace("{{SCRIPT}}", &script);
    let dist = PathBuf::from("dist");
    if dist.exists() {
        std::fs::write(dist.join("index.html"), html).unwrap();
    }
}

fn deploy(bin_name: &str, args: &[String]) {
    let mut w = 600;
    let mut h = 400;
    let mut title = "my app".to_string();
    let mut port = "8080".to_string();
    let mut proj_path = None;
    for arg in args {
        if arg == "--help" {
            println!("{}", DEPLOY.replace("{{BIN}}", bin_name));
            return;
        }
        if let Some(using) = arg.strip_prefix("--using=") {
            proj_path = Some(PathBuf::from(using));
            break;
        }
        if let Some(width) = arg.strip_prefix("--width=") {
            w = width.parse().unwrap();
        }
        if let Some(height) = arg.strip_prefix("--height=") {
            h = height.parse().unwrap();
        }
        if let Some(t) = arg.strip_prefix("--title=") {
            title = t.to_string();
        }
        if let Some(p) = arg.strip_prefix("--port=") {
            port = p.to_string();
        }
    }
    build(&[String::new(), String::new(), "--release".to_string()]);
    let app = APP
        .to_string()
        .replace("{{width}}", &w.to_string())
        .replace("{{height}}", &h.to_string())
        .replace("{{port}}", &format!("\"{}\"", port))
        .replace("{{title}}", &format!("\"{}\"", title));
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to find a Cargo.toml!");
    let pkg: toml::Value = cargo_toml.parse().unwrap();
    let mut crate_name = format!("{}", pkg["package"]["name"]).replace('"', "");
    let cargo_toml = CARGO.to_string().replace("{{crate}}", &crate_name);
    let temp_dir = std::env::temp_dir();
    let proj = if let Some(proj_path) = proj_path {
        let cargo_toml = std::fs::read_to_string(proj_path.join("Cargo.toml"))
            .expect("Failed to find a Cargo.toml!");
        let pkg: toml::Value = cargo_toml.parse().unwrap();
        crate_name = format!("{}", pkg["package"]["name"]).replace('"', "");
        proj_path
    } else {
        temp_dir.join("livid_temp")
    };
    if !proj.exists() {
        let mut cargo = Command::new("cargo");
        cargo.current_dir(&temp_dir);
        cargo.args(["new", "livid_temp"]);
        cargo.spawn().unwrap().wait().unwrap();
        std::fs::write(proj.join("src").join("main.rs"), app).unwrap();
        std::fs::write(proj.join("Cargo.toml"), cargo_toml).unwrap();
    }
    let mut cargo = Command::new("cargo");
    cargo.current_dir(&proj);
    cargo.args(["build", "--release"]);
    cargo.spawn().unwrap().wait().unwrap();
    let cwd = std::env::current_dir().unwrap();
    let bundle = cwd.join("bundle");
    if !bundle.exists() {
        std::fs::create_dir(&bundle).unwrap();
    }
    let exe = if cfg!(target_os = "windows") {
        crate_name.push_str(".exe");
        crate_name
    } else {
        crate_name
    };
    std::fs::copy(
        proj.join("target").join("release").join(&exe),
        bundle.join(exe),
    )
    .unwrap();
    let mut opts = fs_extra::dir::CopyOptions::new();
    opts.overwrite = true;
    opts.copy_inside = true;
    fs_extra::dir::copy(cwd.join("dist"), cwd.join("bundle"), &opts).unwrap();
}

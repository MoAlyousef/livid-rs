use std::path::PathBuf;
use std::process::Command;
use livid_server::Server;

const USAGE: &str = r#"livid {{VERSION}}
Builds and bundles your wasm web app.
USAGE:
    livid <SUBCOMMAND>
SUBCOMMANDS:
    build     Build your wasm web app
    clean     Clean output artifacts
    serve     Serve the generated index.html
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    handle_args(&args);
}

fn handle_args(args: &[String]) {
    if args.len() == 1 {
        help();
        return;
    }
    match args[1].as_str() {
        "build" => build(args),
        "serve" => {
            build(args);
            serve();
        },
        "clean" => clean(),
        "--help" | "--version" => help(),
        _ => help(),
    }
}

fn check_prog(prog: &str) -> bool {
    let mut cmd = Command::new(prog);
    cmd.args(&["--help"]);
    cmd.output().is_ok()
}

fn help() {
    let usage = USAGE.replace("{{VERSION}}", env!("CARGO_PKG_VERSION"));
    println!("{}", usage);
}

fn clean() {
    let mut cargo = Command::new("cargo");
    cargo.args(&["clean"]);
    cargo.spawn().unwrap().wait().unwrap();

    let dist = PathBuf::from("dist");
    if dist.exists() {
        std::fs::remove_dir_all(dist).unwrap();
    }
}

fn serve() {
    println!("Livid server running on http://0.0.0.0:8080!\nServing dist/");
    Server::serve("8080", &std::env::current_dir().unwrap().join("dist"));
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
    path = path.join(&format!("{}.wasm", &crate_name));
    if !check_prog("wasm-bindgen") {
        eprintln!("wasm-bindgen-cli was not found, running a first-time install...");
        let mut cargo = Command::new("cargo");
        cargo.args(&["install", "wasm-bindgen-cli"]);
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
        opt.args(&[&path, "-O3", "-o", &path]);
        opt.spawn().unwrap().wait().unwrap();
    }
    let mut wb = Command::new("wasm-bindgen");
    wb.args(&[
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
        .unwrap_or(HTML.to_string())
        .replace("{{SCRIPT}}", &script);
    let dist = PathBuf::from("dist");
    if dist.exists() {
        std::fs::write(dist.join("index.html"), html).unwrap();
    }
}
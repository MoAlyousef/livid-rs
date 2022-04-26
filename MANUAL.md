# Manual builds

## Building without livid-cli:

- Install wasm-bindgen-cli:

`cargo install wasm-bindgen-cli`

- Create a project:
```toml
[project]
name = "myapp"

[dependencies]
livid = "0.1"
```

```rust,no_run
use livid::{enums::*, prelude::*, *};

fn main() {
    // your frontend code
}
```

- Create a dist/index.html file:
```html
<html>
  <head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  </head>
  <body>
    <script src="./myapp.js"></script>
    <script type="module">
      import init from "./myapp.js";
      init();
    </script>
  </body>
</html>
```

- Build your project with cargo:

`cargo build --release --target wasm32-unknown-unknown`

- Run wasm-bindgen on your generated wasm file

`wasm-bindgen target/wasm32-unknown-unknown/debug/myapp.wasm --out-dir dist --target web --no-typescript --weak-refs`

Notice that the argument weak-refs is passed to wasm-bindgen to enable callback cleanup from the JS side.

This will generate several js glue code inside a `dist` directory, allowing the loading of your wasm binary.

- Serve your index.html using a server of your choosing:

`python3 -m http.server --dir dist`

## Bundling a desktop app without livid-cli
Create a project with a workspace.
```
mkdir myproj
cd myproj
```
Create a Cargo.toml with the following content:
```toml
[workspace]
members = [
    "frontend",
    "backend"
]
```
Create both projects:
```
cargo new frontend
cargo new backend
```
For the backend, add livid-desktop as dependency and in your main.rs:
```rust
use livid_desktop::{App, Settings};

fn main() {
    let a = App::new(Settings {
        w: 600,
        h: 400,
        title: "My App",
        fixed: false,
        port: 8080,
        ..Default::default()
    });
    a.run();
}
```
build normally for your platform:
```
cargo build --release
```

For your frontend project, follow the `Building without livid-cli` section.

After building both projects, deploy your backend app with the frontend generated html, js and wasm files.

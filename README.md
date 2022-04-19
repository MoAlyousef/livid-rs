# livid-rs

livid-rs is a Rust framework for creating frontend and backend wasm web apps (targetting browsers and desktop).
It's composed of:
- livid:
A lightweight frontend Rust crate for creating web apps via webassembly. It provides the following advantagges:
    - Backend agnostic.
    - Thin wrapper around web-sys
    - No vdom.
    - No macros!

- livid-cli:
A lightweight build/dev tool which allows compiling your frontend into a web app or desktop app (via `livid deploy`). It can be invoked from the command line:
```
livid build
livid serve
# if targetting desktop
livid deploy 
```

- livid-desktop:
A minimal webview implementation in Rust, which allows deploying your wasm web app as desktop apps. It's also frontend agnostic.

- livid-server:
An internal server used in livid-cli and livid-desktop

- Examples:
    - Different architectures (low-level, widgets api, elm architecture).
    - Fullstack livid application
    - livid frontend with a wry backend.
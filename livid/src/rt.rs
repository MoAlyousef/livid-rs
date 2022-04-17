use wasm_bindgen::prelude::*;
use js_sys::Reflect;

pub fn call(method: &str, args: &[&str]) -> Result<JsValue, JsValue> {
    let obj = web_sys::window().unwrap();
    let method: js_sys::Function = Reflect::get(&obj, &JsValue::from_str(method)).unwrap().into();
    let arguments = js_sys::Array::new();
    for arg in args {
        arguments.push(&JsValue::from_str(arg));
    }
    Reflect::apply(&method, &obj, &arguments)
}

pub fn alert(msg: &str) {
    web_sys::window().unwrap().alert_with_message(msg).unwrap();
}

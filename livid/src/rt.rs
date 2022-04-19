use wasm_bindgen::prelude::*;
use js_sys::Reflect;

pub fn call(method: &str, args: &[&str]) -> Result<JsValue, JsValue> {
    let obj = web_sys::window().unwrap();
    let method: js_sys::Function = Reflect::get(&obj, &method.into())?.into();
    let arguments = js_sys::Array::new();
    for arg in args {
        arguments.push(&JsValue::from_str(arg));
    }
    Reflect::apply(&method, &obj, &arguments)
}

pub fn post_message(arg: &str) -> Result<JsValue, JsValue> {
    let obj = web_sys::window().unwrap();
    let ipc: JsValue = Reflect::get(&obj, &"ipc".into())?;
    let method: js_sys::Function = Reflect::get(&ipc, &"postMessage".into())?.into();
    let arguments = js_sys::Array::new();
    arguments.push(&JsValue::from_str(arg));
    Reflect::apply(&method, &obj, &arguments)
}

pub fn get_variable(name: &str) -> Result<JsValue, JsValue> {
    let obj = web_sys::window().unwrap();
    let name: JsValue = Reflect::get(&obj, &name.into())?;
    Ok(name)
}

pub fn alert(msg: &str) {
    web_sys::window().unwrap().alert_with_message(msg).unwrap();
}

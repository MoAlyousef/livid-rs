use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

/**
    Convenience function to convert hex to rgb.
    Example:
    ```rust,no_run
    use livid::utils::hex2rgb;
    let (r, g, b) = hex2rgb(0x000000);
    ```
*/
pub const fn hex2rgb(val: u32) -> (u8, u8, u8) {
    let r = ((val >> 16) & 0xff) as u8;
    let g = ((val >> 8) & 0xff) as u8;
    let b = (val & 0xff) as u8;
    (r, g, b)
}

pub fn set_interval<F: 'static + FnMut()>(timeout_ms: u32, mut cb: F) -> i32 {
    let cb1 = Closure::wrap(Box::new(move || {
        cb();
    }) as Box<dyn FnMut()>);
    let ret = crate::document::Document::window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            cb1.as_ref().unchecked_ref(),
            timeout_ms as i32,
        )
        .expect_throw("should register `setTimeout` OK");
    cb1.forget();
    ret
}

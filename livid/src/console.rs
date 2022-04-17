/// A wrapper around the web console
pub struct Console;

impl Console {
    /// console.log()
    pub fn log(s: &str) {
        web_sys::console::log_1(&s.into());
    }
    /// console.warn()
    pub fn warn(s: &str) {
        web_sys::console::warn_1(&s.into());
    }
    /// console.error()
    pub fn error(s: &str) {
        web_sys::console::error_1(&s.into());
    }
    /// console.group()
    pub fn group(s: &str) {
        web_sys::console::group_1(&s.into());
    }
    /// console.clear()
    pub fn clear() {
        web_sys::console::clear();
    }
}

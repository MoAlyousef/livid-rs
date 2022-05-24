pub use crate::error::*;
pub use crate::traits::*;
pub trait SvgDrawer {
    fn draw_svg(&self, svg: &str, dx: f64, dy: f64, dw: f64, dh: f64);
}
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

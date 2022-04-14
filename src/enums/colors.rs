use crate::prelude::{LividError, LividErrorKind};

#[derive(Debug, Copy, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, Copy, Clone)]
pub struct Rgba(pub u8, pub u8, pub u8, pub u8);

#[derive(Debug, Copy, Clone)]
pub enum Color {
    IndianRed,
    LightCoral,
    Salmon,
    DarkSalmon,
    LightSalmon,
    Crimson,
    Red,
    FireBrick,
    DarkRed,
    Pink,
    LightPink,
    HotPink,
    DeepPink,
    MediumVioletRed,
    PaleVioletRed,
    Coral,
    Tomato,
    OrangeRed,
    DarkOrange,
    Orange,
    Gold,
    Yellow,
    LightYellow,
    LemonChiffon,
    LightGoldenrodYellow,
    PapayaWhip,
    Moccasin,
    PeachPuff,
    PaleGoldenrod,
    Khaki,
    DarkKhaki,
    Lavender,
    Thistle,
    Plum,
    Violet,
    Orchid,
    Fuchsia,
    Magenta,
    MediumOrchid,
    MediumPurple,
    RebeccaPurple,
    BlueViolet,
    DarkViolet,
    DarkOrchid,
    DarkMagenta,
    Purple,
    Indigo,
    SlateBlue,
    DarkSlateBlue,
    MediumSlateBlue,
    GreenYellow,
    Chartreuse,
    LawnGreen,
    Lime,
    LimeGreen,
    PaleGreen,
    LightGreen,
    MediumSpringGreen,
    SpringGreen,
    MediumSeaGreen,
    SeaGreen,
    ForestGreen,
    Green,
    DarkGreen,
    YellowGreen,
    OliveDrab,
    Olive,
    DarkOliveGreen,
    MediumAquamarine,
    DarkSeaGreen,
    LightSeaGreen,
    DarkCyan,
    Teal,
    Aqua,
    Cyan,
    LightCyan,
    PaleTurquoise,
    Aquamarine,
    Turquoise,
    MediumTurquoise,
    DarkTurquoise,
    CadetBlue,
    SteelBlue,
    LightSteelBlue,
    PowderBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    DeepSkyBlue,
    DodgerBlue,
    CornflowerBlue,
    RoyalBlue,
    Blue,
    MediumBlue,
    DarkBlue,
    Navy,
    MidnightBlue,
    Cornsilk,
    BlanchedAlmond,
    Bisque,
    NavajoWhite,
    Wheat,
    BurlyWood,
    Tan,
    RosyBrown,
    SandyBrown,
    Goldenrod,
    DarkGoldenrod,
    Peru,
    Chocolate,
    SaddleBrown,
    Sienna,
    Brown,
    Maroon,
    White,
    Snow,
    HoneyDew,
    MintCream,
    Azure,
    AliceBlue,
    GhostWhite,
    WhiteSmoke,
    SeaShell,
    Beige,
    OldLace,
    FloralWhite,
    Ivory,
    AntiqueWhite,
    Linen,
    LavenderBlush,
    MistyRose,
    Gainsboro,
    LightGray,
    Silver,
    DarkGray,
    Gray,
    DimGray,
    LightSlateGray,
    SlateGray,
    DarkSlateGray,
    Black,
    Rgb(Rgb),
    Rgba(Rgba),
}

impl Color {
    pub fn to_str(self) -> String {
        if let Color::Rgb(rgb) = self {
            let Rgb(r, g, b) = rgb;
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        } else if let Color::Rgba(rgba) = self {
            let Rgba(r, g, b, a) = rgb;
            format!("rgba({}, {}, {}, {})", r, g, b, a)
        } else {
            format!("{:?}", self)
        }
    }
    /// Return a Color from a hex color format (`#xxxxxx`)
    pub fn from_hex_str(col: &str) -> Result<Color, LividError> {
        if !col.starts_with('#') || col.len() != 7 {
            Err(LividError::Internal(LividErrorKind::InvalidColor))
        } else {
            Ok(Color::from_hex(u32::from_str_radix(&col[1..7], 16)?))
        }
    }
    pub fn from_hex(val: u32) -> Color {
        let (r, g, b) = crate::utils::hex2rgb(val);
        Color::Rgb(Rgb(r, g, b))
    }
}

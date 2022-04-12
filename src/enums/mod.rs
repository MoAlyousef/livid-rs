mod colors;
mod events;
mod frames;
mod styles;
mod types;

pub use colors::*;
pub use events::*;
pub use frames::*;
pub use styles::*;
pub use types::*;

#[derive(Debug, Copy, Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
    Initial,
    Inherit,
}

impl TextAlign {
    pub fn to_str(self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    pub fn to_str(self) -> String {
        format!("{:?}", self).to_ascii_lowercase()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AlignContent {
    Stretch,
    Center,
    FlexStart,
    FlexEnd,
    SpaceBetween,
    SpaceAround,
    Initial,
    Inherit
}

impl AlignContent {
    pub fn to_str(self) -> &'static str {
        use AlignContent::*;
        match self {
            Stretch => "stretch",
            Center => "center",
            FlexStart => "flex-start",
            FlexEnd => "flex-end",
            SpaceBetween => "space-between",
            SpaceAround => "space-around",
            Initial => "initial",
            Inherit => "inherit",
        }
    }
}
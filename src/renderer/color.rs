// dacho/src/renderer/color.rs

pub type ColorData = (f32, f32, f32);

pub struct Color;

impl Color {
    pub const RED:     ColorData = (1.0, 0.0, 0.0);
    pub const GREEN:   ColorData = (0.0, 1.0, 0.0);
    pub const BLUE:    ColorData = (0.0, 0.0, 1.0);
    pub const WHITE:   ColorData = (1.0, 1.0, 1.0);
    pub const CYAN:    ColorData = (0.0, 1.0, 1.0);
    pub const MAGENTA: ColorData = (1.0, 0.0, 1.0);
    pub const YELLOW:  ColorData = (1.0, 1.0, 0.0);
    pub const BLACK:   ColorData = (0.0, 0.0, 0.0);
}


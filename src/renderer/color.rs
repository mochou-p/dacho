// dacho/src/renderer/color.rs

pub type ColorData = (f32, f32, f32);

pub struct Color;

#[allow(dead_code)]
impl Color {
    pub const WHITE:   ColorData = (1.00, 1.00, 1.00);
    pub const LIGHTER: ColorData = (0.91, 0.91, 0.91);
    pub const LIGHT:   ColorData = (0.75, 0.75, 0.75);
    pub const GRAY:    ColorData = (0.50, 0.50, 0.50);
    pub const DARK:    ColorData = (0.25, 0.25, 0.25);
    pub const DARKER:  ColorData = (0.09, 0.09, 0.09);
    pub const BLACK:   ColorData = (0.00, 0.00, 0.00);

    pub const RED:     ColorData = (1.00, 0.00, 0.00);
    pub const ORANGE:  ColorData = (1.00, 0.50, 0.00);
    pub const YELLOW:  ColorData = (1.00, 1.00, 0.00);
    pub const LIME:    ColorData = (0.50, 1.00, 0.00);
    pub const GREEN:   ColorData = (0.00, 1.00, 0.00);
    pub const AQUA:    ColorData = (0.00, 1.00, 0.50);
    pub const CYAN:    ColorData = (0.00, 1.00, 1.00);
    pub const SKY:     ColorData = (0.00, 0.50, 1.00);
    pub const BLUE:    ColorData = (0.00, 0.00, 1.00);
    pub const PURPLE:  ColorData = (0.50, 0.00, 1.00);
    pub const MAGENTA: ColorData = (1.00, 0.00, 1.00);
    pub const CANDY:   ColorData = (1.00, 0.00, 0.50);
}


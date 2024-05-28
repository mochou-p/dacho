// dacho/src/prelude/colors.rs

use super::types::V3;

pub struct Color;

#[allow(dead_code)]
impl Color {
    pub const BLACK:   V3 = V3::new(0.00, 0.00, 0.00);
    pub const DARK:    V3 = V3::new(0.15, 0.15, 0.15);
    pub const GRAY:    V3 = V3::new(0.50, 0.50, 0.50);
    pub const LIGHT:   V3 = V3::new(0.85, 0.85, 0.85);
    pub const WHITE:   V3 = V3::new(1.00, 1.00, 1.00);

    pub const RED:     V3 = V3::new(1.00, 0.00, 0.00);
    pub const ORANGE:  V3 = V3::new(1.00, 0.50, 0.00);
    pub const YELLOW:  V3 = V3::new(1.00, 1.00, 0.00);
    pub const LIME:    V3 = V3::new(0.50, 1.00, 0.00);
    pub const GREEN:   V3 = V3::new(0.00, 1.00, 0.00);
    pub const TEAL:    V3 = V3::new(0.00, 1.00, 0.50);
    pub const CYAN:    V3 = V3::new(0.00, 1.00, 1.00);
    pub const SKY:     V3 = V3::new(0.00, 0.50, 1.00);
    pub const BLUE:    V3 = V3::new(0.00, 0.00, 1.00);
    pub const PURPLE:  V3 = V3::new(0.50, 0.00, 1.00);
    pub const MAGENTA: V3 = V3::new(1.00, 0.00, 1.00);
    pub const CANDY:   V3 = V3::new(1.00, 0.00, 0.50);

    pub const GREY:    V3 = Self::GRAY;
}


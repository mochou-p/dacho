// dacho/src/prelude/colors.rs

use super::types::V3;

pub struct Color;

#[allow(dead_code)]
impl Color {
    pub const BLACK:         V3 = V3::new(0.00, 0.00, 0.00);
    pub const DARK:          V3 = V3::new(0.15, 0.15, 0.15);
    pub const GRAY:          V3 = V3::new(0.50, 0.50, 0.50);
    pub const LIGHT:         V3 = V3::new(0.85, 0.85, 0.85);
    pub const WHITE:         V3 = V3::new(1.00, 1.00, 1.00);

    pub const RED:           V3 = V3::new(1.00, 0.00, 0.00);
    pub const ORANGE:        V3 = V3::new(1.00, 0.50, 0.00);
    pub const YELLOW:        V3 = V3::new(1.00, 1.00, 0.00);
    pub const LIME:          V3 = V3::new(0.50, 1.00, 0.00);
    pub const GREEN:         V3 = V3::new(0.00, 1.00, 0.00);
    pub const TEAL:          V3 = V3::new(0.00, 1.00, 0.50);
    pub const CYAN:          V3 = V3::new(0.00, 1.00, 1.00);
    pub const SKY:           V3 = V3::new(0.00, 0.50, 1.00);
    pub const BLUE:          V3 = V3::new(0.00, 0.00, 1.00);
    pub const PURPLE:        V3 = V3::new(0.50, 0.00, 1.00);
    pub const MAGENTA:       V3 = V3::new(1.00, 0.00, 1.00);
    pub const CANDY:         V3 = V3::new(1.00, 0.00, 0.50);

    pub const DARK_RED:      V3 = V3::new(0.30, 0.00, 0.00);
    pub const DARK_ORANGE:   V3 = V3::new(0.30, 0.15, 0.00);
    pub const DARK_YELLOW:   V3 = V3::new(0.30, 0.30, 0.00);
    pub const DARK_LIME:     V3 = V3::new(0.15, 0.30, 0.00);
    pub const DARK_GREEN:    V3 = V3::new(0.00, 0.30, 0.00);
    pub const DARK_TEAL:     V3 = V3::new(0.00, 0.30, 0.15);
    pub const DARK_CYAN:     V3 = V3::new(0.00, 0.30, 0.30);
    pub const DARK_SKY:      V3 = V3::new(0.00, 0.15, 0.30);
    pub const DARK_BLUE:     V3 = V3::new(0.00, 0.00, 0.30);
    pub const DARK_PURPLE:   V3 = V3::new(0.15, 0.00, 0.30);
    pub const DARK_MAGENTA:  V3 = V3::new(0.30, 0.00, 0.30);
    pub const DARK_CANDY:    V3 = V3::new(0.30, 0.00, 0.15);

    // broken pbr, diffuse is inverted
    pub const LIGHT_RED:     V3 = V3::new(1.00, 0.70, 0.70);
    pub const LIGHT_ORANGE:  V3 = V3::new(1.00, 0.85, 0.70);
    pub const LIGHT_YELLOW:  V3 = V3::new(1.00, 1.00, 0.70);
    pub const LIGHT_LIME:    V3 = V3::new(0.85, 1.00, 0.70);
    pub const LIGHT_GREEN:   V3 = V3::new(0.70, 1.00, 0.70);
    pub const LIGHT_TEAL:    V3 = V3::new(0.70, 1.00, 0.85);
    pub const LIGHT_CYAN:    V3 = V3::new(0.70, 1.00, 1.00);
    pub const LIGHT_SKY:     V3 = V3::new(0.70, 0.85, 1.00);
    pub const LIGHT_BLUE:    V3 = V3::new(0.70, 0.70, 1.00);
    pub const LIGHT_PURPLE:  V3 = V3::new(0.85, 0.70, 1.00);
    pub const LIGHT_MAGENTA: V3 = V3::new(1.00, 0.70, 1.00);
    pub const LIGHT_CANDY:   V3 = V3::new(1.00, 0.70, 0.85);

    pub const GREY:          V3 = Self::GRAY;
}


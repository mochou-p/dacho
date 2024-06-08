// dacho/src/prelude/colors.rs

// super
use super::types::V3;

pub struct Color;

impl Color {
    pub const BLACK:         V3 = V3::new(0.01, 0.01, 0.01); // #020202
    pub const DARK:          V3 = V3::new(0.15, 0.15, 0.15); // #262626
    pub const GRAY:          V3 = V3::new(0.50, 0.50, 0.50); // #7f7f7f
    pub const LIGHT:         V3 = V3::new(0.85, 0.85, 0.85); // #d8d8d8
    pub const WHITE:         V3 = V3::new(1.00, 1.00, 1.00); // #ffffff

    pub const RED:           V3 = V3::new(1.00, 0.00, 0.00); // #ff0000
    pub const ORANGE:        V3 = V3::new(1.00, 0.50, 0.00); // #ff7f00
    pub const YELLOW:        V3 = V3::new(1.00, 1.00, 0.00); // #ffff00
    pub const LIME:          V3 = V3::new(0.50, 1.00, 0.00); // #7fff00
    pub const GREEN:         V3 = V3::new(0.00, 1.00, 0.00); // #00ff00
    pub const TEAL:          V3 = V3::new(0.00, 1.00, 0.50); // #00ff7f
    pub const CYAN:          V3 = V3::new(0.00, 1.00, 1.00); // #00ffff
    pub const SKY:           V3 = V3::new(0.00, 0.50, 1.00); // #007fff
    pub const BLUE:          V3 = V3::new(0.00, 0.00, 1.00); // #0000ff
    pub const PURPLE:        V3 = V3::new(0.50, 0.00, 1.00); // #7f00ff
    pub const MAGENTA:       V3 = V3::new(1.00, 0.00, 1.00); // #ff00ff
    pub const CANDY:         V3 = V3::new(1.00, 0.00, 0.50); // #ff007f

    pub const DARK_RED:      V3 = V3::new(0.30, 0.00, 0.00); // #4c0000
    pub const DARK_ORANGE:   V3 = V3::new(0.30, 0.15, 0.00); // #4c2600
    pub const DARK_YELLOW:   V3 = V3::new(0.30, 0.30, 0.00); // #4c4c00
    pub const DARK_LIME:     V3 = V3::new(0.15, 0.30, 0.00); // #264c00
    pub const DARK_GREEN:    V3 = V3::new(0.00, 0.30, 0.00); // #004c00
    pub const DARK_TEAL:     V3 = V3::new(0.00, 0.30, 0.15); // #004c26
    pub const DARK_CYAN:     V3 = V3::new(0.00, 0.30, 0.30); // #004c4c
    pub const DARK_SKY:      V3 = V3::new(0.00, 0.15, 0.30); // #00264c
    pub const DARK_BLUE:     V3 = V3::new(0.00, 0.00, 0.30); // #00004c
    pub const DARK_PURPLE:   V3 = V3::new(0.15, 0.00, 0.30); // #26004c
    pub const DARK_MAGENTA:  V3 = V3::new(0.30, 0.00, 0.30); // #4c004c
    pub const DARK_CANDY:    V3 = V3::new(0.30, 0.00, 0.15); // #4c0026

    pub const LIGHT_RED:     V3 = V3::new(1.00, 0.70, 0.70); // #ffb2b2
    pub const LIGHT_ORANGE:  V3 = V3::new(1.00, 0.85, 0.70); // #ffd8b2
    pub const LIGHT_YELLOW:  V3 = V3::new(1.00, 1.00, 0.70); // #ffffb2
    pub const LIGHT_LIME:    V3 = V3::new(0.85, 1.00, 0.70); // #d8ffb2
    pub const LIGHT_GREEN:   V3 = V3::new(0.70, 1.00, 0.70); // #b2ffb2
    pub const LIGHT_TEAL:    V3 = V3::new(0.70, 1.00, 0.85); // #b2ffd8
    pub const LIGHT_CYAN:    V3 = V3::new(0.70, 1.00, 1.00); // #b2ffff
    pub const LIGHT_SKY:     V3 = V3::new(0.70, 0.85, 1.00); // #b2d8ff
    pub const LIGHT_BLUE:    V3 = V3::new(0.70, 0.70, 1.00); // #b2b2ff
    pub const LIGHT_PURPLE:  V3 = V3::new(0.85, 0.70, 1.00); // #d8b2ff
    pub const LIGHT_MAGENTA: V3 = V3::new(1.00, 0.70, 1.00); // #ffb2ff
    pub const LIGHT_CANDY:   V3 = V3::new(1.00, 0.70, 0.85); // #ffb2d8

    pub const GREY:          V3 = Self::GRAY;

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> V3 {
        Self::WHITE
    }
}


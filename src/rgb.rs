use crate::*;

/// RGB color, with u8 components
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgb {
    /// red
    pub r: u8,
    /// green
    pub g: u8,
    /// blue
    pub b: u8,
}

impl Rgb {
    /// Create a new RGB color from its components
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn to_ansi(self) -> AnsiColor {
        AnsiColor {
            code: ansi_colours::ansi256_from_rgb((self.r, self.g, self.b)),
        }
    }
    #[allow(clippy::float_cmp)]
    pub fn to_hsl(self) -> Hsl {
        let (r, g, b) = (self.rp(), self.gp(), self.bp());
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);

        let l = 0.5 * (max + min);

        if min == max {
            // gray level
            return Hsl::new(0.0, 0.0, l);
        }

        let h = if max == r {
            60.0 * (g - b) / (max - min)
        } else if max == g {
            60.0 * (b - r) / (max - min) + 120.0
        } else if max == b {
            60.0 * (r - g) / (max - min) + 240.0
        } else {
            0.0
        };
        let h = (h + 360.0) % 360.0;

        let s = if 0.0 < l && l <= 0.5 {
            (max - min) / (2.0 * l)
        } else {
            (max - min) / (2.0 - 2.0 * l)
        };

        Hsl { h, s, l }
    }
    /// red part in `[0,1]`
    pub fn rp(self) -> f32 {
        self.r as f32 / 256f32
    }
    /// green part in `[0,1]`
    pub fn gp(self) -> f32 {
        self.g as f32 / 256f32
    }
    /// blue part in `[0,1]`
    pub fn bp(self) -> f32 {
        self.b as f32 / 256f32
    }
    /// Compute the Luma value characterizing the "light" of the color,
    /// going from 0 (black) to 1 (white).
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Luma_(video)>
    pub fn luma(self) -> f32 {
        0.2627 * self.rp() + 0.6780 * self.gp() + 0.0593 * self.bp()
    }
}

impl From<(f32, f32, f32)> for Rgb {
    /// Convert from a (r,g,b) float tupples with components in [0,1[
    fn from(c: (f32, f32, f32)) -> Self {
        debug_assert!(c.0<=1.0);
        debug_assert!(c.1<=1.0);
        debug_assert!(c.2<=1.0);
        Rgb::new(
            (c.0 * 255.0).round() as u8,
            (c.1 * 255.0).round() as u8,
            (c.2 * 255.0).round() as u8,
        )
    }
}

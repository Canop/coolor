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
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub const fn is_grey(self) -> bool {
        self.r == self.g && self.g == self.b
    }
    /// Return the nearest ANSI color
    ///
    /// This is a slow function as it literally tries all
    /// ANSI colors and picks the nearest one.
    /// The ansi->rgb->ansi round trip is guaranteed to
    /// always fall on the first color.
    pub fn to_ansi_slow(self) -> AnsiColor {
        let mut best = AnsiColor { code: 16 };
        let mut smallest_distance: f32 = self.distance_to(best.to_rgb());
        for code in 17..=255 {
            let color = AnsiColor { code };
            let distance = self.distance_to(color.to_rgb());
            if distance < smallest_distance {
                best = color;
                smallest_distance = distance;
            }
        }
        best
    }
    /// Return the nearest ANSI color
    ///
    /// This uses the excellent ansi_colours crate, as it's
    /// very fast and gives rather good conversions. It doesn't
    /// ensure a perfect ansi->rgb->ansi round trip: you don't
    /// always fall on the first color.
    pub fn to_ansi(self) -> AnsiColor {
        AnsiColor::new(ansi_colours::ansi256_from_rgb((self.r, self.g, self.b)))
    }
    pub fn mix(c1: Self, w1: f32, c2: Self, w2: f32) -> Self {
        debug_assert!(w1 + w2 > 0.0);
        let (r1, g1, b1) = c1.parts();
        let (r2, g2, b2) = c2.parts();
        let r = (w1 * r1 + w2 * r2) / (w1 + w2);
        let g = (w1 * g1 + w2 * g2) / (w1 + w2);
        let b = (w1 * b1 + w2 * b2) / (w1 + w2);
        (r, g, b).into()
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
    pub fn parts(self) -> (f32, f32, f32) {
        (self.rp(), self.gp(), self.bp())
    }
    /// Compute the Luma value characterizing the "light" of the color,
    /// going from 0 (black) to 1 (white).
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Luma_(video)>
    pub fn luma(self) -> f32 {
        0.2627 * self.rp() + 0.6780 * self.gp() + 0.0593 * self.bp()
    }
    /// tentatively perceptual distance between two RGB colors
    /// (adapted from the ansi_colours crate, by mina86, who adapted
    /// a formula found at https://www.compuphase.com/cmetric.htm)
    pub fn distance_to<O: Into<Rgb>>(self, other: O) -> f32 {
        let other = other.into();
        let r_sum = self.r as f32 + other.r as f32;
        let r = self.r as f32 - other.r as f32;
        let g = self.g as f32 - other.g as f32;
        let b = self.b as f32 - other.b as f32;
        (1024.0 + r_sum) * r * r + 2048.0 * g * g + (1534.0 - r_sum) * b * b
    }
}

pub fn r255(v: f32) -> u8 {
    (v * 255.0) as u8
}

impl From<(f32, f32, f32)> for Rgb {
    /// Convert from a (r,g,b) float tupples with components in [0,1[
    fn from(c: (f32, f32, f32)) -> Self {
        debug_assert!(c.0 <= 1.0);
        debug_assert!(c.1 <= 1.0);
        debug_assert!(c.2 <= 1.0);
        Rgb::new(r255(c.0), r255(c.1), r255(c.2))
    }
}

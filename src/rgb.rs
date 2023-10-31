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
    #[inline]
    pub fn nearest_ansi_in_range(self, min: u8, max: u8) -> AnsiColor {
        let mut best = AnsiColor { code: min };
        let mut smallest_distance: f32 = self.distance_to(best.to_rgb());
        for code in min+1..=max {
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
    /// The ansi->rgb->ansi round trip is guaranteed to
    /// always fall on the first color.
    pub fn to_ansi(self) -> AnsiColor {
        if self.r == self.g && self.g == self.b {
            AnsiColor { code: GREY_TO_ANSI[self.r as usize] }
        } else if self.r < 108 {
            if self.r < 41 {
                self.nearest_ansi_in_range(17, 51)
            } else {
                self.nearest_ansi_in_range(52, 87)
            }
        } else if self.r < 195 {
            if self.r < 151 {
                self.nearest_ansi_in_range(88, 123)
            } else {
                self.nearest_ansi_in_range(124, 159)
            }
        } else {
            if self.r < 235 {
                self.nearest_ansi_in_range(160, 195)
            } else {
                self.nearest_ansi_in_range(196, 230)
            }
        }
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
    #[inline(always)]
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

pub const GREY_TO_ANSI: &[u8] = &[
    16,
    16,
    16,
    16,
    16,
    232,
    232,
    232,
    232,
    232,
    232,
    232,
    232,
    232,
    233,
    233,
    233,
    233,
    233,
    233,
    233,
    233,
    233,
    233,
    234,
    234,
    234,
    234,
    234,
    234,
    234,
    234,
    234,
    234,
    235,
    235,
    235,
    235,
    235,
    235,
    235,
    235,
    235,
    235,
    236,
    236,
    236,
    236,
    236,
    236,
    236,
    236,
    236,
    236,
    237,
    237,
    237,
    237,
    237,
    237,
    237,
    237,
    237,
    237,
    238,
    238,
    238,
    238,
    238,
    238,
    238,
    238,
    238,
    238,
    239,
    239,
    239,
    239,
    239,
    239,
    239,
    239,
    239,
    239,
    240,
    240,
    240,
    240,
    240,
    240,
    240,
    240,
    59,
    59,
    59,
    59,
    241,
    241,
    241,
    241,
    242,
    242,
    242,
    242,
    242,
    242,
    242,
    242,
    242,
    242,
    242,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    243,
    244,
    244,
    244,
    244,
    244,
    244,
    244,
    244,
    102,
    102,
    102,
    102,
    102,
    245,
    245,
    245,
    245,
    245,
    245,
    245,
    246,
    246,
    246,
    246,
    246,
    246,
    246,
    246,
    246,
    246,
    247,
    247,
    247,
    247,
    247,
    247,
    247,
    247,
    247,
    247,
    248,
    248,
    248,
    248,
    248,
    248,
    248,
    248,
    145,
    145,
    145,
    145,
    145,
    249,
    249,
    249,
    249,
    249,
    249,
    249,
    250,
    250,
    250,
    250,
    250,
    250,
    250,
    250,
    250,
    250,
    251,
    251,
    251,
    251,
    251,
    251,
    251,
    251,
    251,
    251,
    252,
    252,
    252,
    252,
    252,
    252,
    252,
    252,
    188,
    188,
    188,
    188,
    188,
    253,
    253,
    253,
    253,
    253,
    253,
    253,
    254,
    254,
    254,
    254,
    254,
    254,
    254,
    254,
    254,
    254,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    255,
    231,
    231,
    231,
    231,
    231,
    231,
    231,
    231,
    231,
];



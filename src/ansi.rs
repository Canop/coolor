use crate::*;

/// 8-bit Ansi Color Code
///
/// See <https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnsiColor {
    pub code: u8,
}

impl AnsiColor {
    pub fn new(code: u8) -> Self {
        Self { code }
    }
    pub fn to_hsl(self) -> Hsl {
        self.to_rgb().to_hsl()
    }
    pub fn to_rgb(self) -> Rgb {
        let (r, g, b) = ansi_colours::rgb_from_ansi256(self.code);
        Rgb { r, g, b }
    }
    pub fn with_luminosity_change(self, delta_luminosity: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.l = (hsl.l + delta_luminosity).min(1.0).max(0.0);
        hsl.to_ansi()
    }
    pub fn with_luminosity(self, l: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.l = l;
        hsl.to_ansi()
    }
    pub fn with_saturation_change(self, delta_saturation: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.s = (hsl.s + delta_saturation).min(1.0).max(0.0);
        hsl.to_ansi()
    }
    pub fn with_saturation(self, s: f32) -> Self {
        let mut hsl = self.to_hsl();
        hsl.s = s;
        hsl.to_ansi()
    }
}


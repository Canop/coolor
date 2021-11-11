use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Ansi(AnsiColor),
    Hsl(Hsl),
    Rgb(Rgb),
}

impl Color {
    pub fn ansi(self) -> AnsiColor {
        match self {
            Self::Ansi(ansi) => ansi,
            Self::Hsl(hsl) => hsl.to_ansi(),
            Self::Rgb(rgb) => rgb.to_ansi(),
        }
    }
    pub fn hsl(self) -> Hsl {
        match self {
            Self::Ansi(ansi) => ansi.to_hsl(),
            Self::Hsl(hsl) => hsl,
            Self::Rgb(rgb) => rgb.to_hsl(),
        }
    }
    pub fn rgb(self) -> Rgb {
        match self {
            Self::Ansi(ansi) => ansi.to_rgb(),
            Self::Hsl(hsl) => hsl.to_rgb(),
            Self::Rgb(rgb) => rgb,
        }
    }
    pub fn luma(self) -> f32 {
        self.rgb().luma()
    }
    /// compute a natural feeling intermediate between two colors
    pub fn blend<C1: Into<Hsl>, C2: Into<Hsl>>(c1: C1, w1: f32, c2: C2, w2: f32) -> Self {
        debug_assert!(w1 + w2 > 0.0);
        let hsl1: Hsl = c1.into();
        let hsl2: Hsl = c2.into();
        let mixed_hsl = Hsl::mix(hsl1, w1, hsl2, w2);
        let rgb1: Rgb = hsl1.to_rgb();
        let rgb2: Rgb = hsl2.to_rgb();
        let mixed_rgb = Rgb::mix(rgb1, w1, rgb2, w2);
        let mixed_rgb_hsl = mixed_rgb.to_hsl();
        let mixed = Hsl::mix(mixed_hsl, 0.5, mixed_rgb_hsl, 0.5);
        Hsl {
            h: mixed_rgb_hsl.h, // hue blending done only on rgb space
            s: mixed.s, // saturation is mix between RGB computation and HSL one
            l: mixed.l, // luminosity is mix between RGB computation and HSL one
        }.into()
    }
}

impl From<AnsiColor> for Color {
    fn from(ansi: AnsiColor) -> Self {
        Self::Ansi(ansi)
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        Self::Rgb(rgb)
    }
}

impl From<Hsl> for Color {
    fn from(rgb: Hsl) -> Self {
        Self::Hsl(rgb)
    }
}

/// check going from ansi to rgb and back makes us fall on the first color
#[test]
fn test_ansi_to_rgb_to_ansi() {
    // we don't check the range 0..16 as it's made of colors
    // which are also in the 16..255 range
    for code in 16..=255 {
        let c1 = AnsiColor { code };
        let c2 = c1.to_rgb();
        let c3 = c2.to_ansi();
        assert_eq!(c1, c3);
    }
}
/// check going from ansi to hsl and back makes us fall on the first color
#[test]
fn test_ansi_to_hsl_to_ansi() {
    // we don't check the range 0..16 as it's made of colors
    // which are also in the 16..255 range
    for code in 16..=255 {
        let c1 = AnsiColor { code };
        let c2 = c1.to_hsl();
        let c3 = c2.to_ansi();
        assert_eq!(c1, c3);
    }
}
#[test]
fn test_rgb_to_hsl() {
    assert!(Rgb::new(255, 0, 0).to_hsl().near(Hsl::new(0.0, 1.0, 0.5))); // red
    assert!(Rgb::new(255, 255, 0).to_hsl().near(Hsl::new(60.0, 1.0, 0.5))); // yellow
    assert!(Rgb::new(255, 255, 255).to_hsl().near(Hsl::new(0.0, 0.0, 1.0))); // white
}
/// check going from hsl to rgb and back makes us fall on the first color (or not too far)
#[test]
fn test_hsl_to_rgb_to_hsl() {
    let red = Hsl::new(0.0, 1.0, 0.5);
    let yellow = Hsl::new(60.0, 1.0, 0.5);
    let white = Hsl::new(0.0, 0.0, 1.0);
    assert!(red.to_rgb().to_hsl().near(red));
    assert!(yellow.to_rgb().to_hsl().near(yellow));
    assert!(white.to_rgb().to_hsl().near(white));
}


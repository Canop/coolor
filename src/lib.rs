/*!

Definition of ANSI, RGB and HSL color types and all the conversions between them.

There are many other color conversion crates.
This one is useful when you're interested into variations of an ANSI color for your TUI application, for example fading, lightening, darkening, with compatibility with terminals which don't support RGB.

The included example shows darkened and lightened variants of all 240 ANSI colors, with all variants still ANSI colors.

*/

mod ansi;
mod color;
mod error;
mod hsl;
mod rgb;

pub use {
    ansi::*,
    color::*,
    error::*,
    hsl::*,
    rgb::*,
};


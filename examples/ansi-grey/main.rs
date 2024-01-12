//! Run this with
//!     cargo run --example ansi-grey --features=crossterm
use {
    coolor::*,
    crossterm::style::{Color as CC, Stylize},
};

/// Return the hexa notation commonly used for the web
pub fn to_hexa(rgb: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

fn print_color(c: Color) {
    let cc: CC = c.into();
    print!("{}", "██████".with(cc));
}

fn show(rgb: Rgb, ansi: AnsiColor) {
    print!(" {} ", to_hexa(rgb));
    print_color(ansi.into());
    print!("  {:>3}   {:>3}", ansi.code, rgb.r);
    println!();
}

fn main() {
    println!("All the ANSI colors with r=g=b");
    println!("  hexa          ansi r=g=b");
    for code in 0..=255 {
        let ansi = AnsiColor { code };
        let rgb = ansi.to_rgb();
        if rgb.r == rgb.g && rgb.g == rgb.b {
            show(rgb, ansi);
        }
    }
    println!();
}

//! Run this with
//!     cargo run --features crossterm --example rgb-to-ansi
use {
    coolor::*,
    crossterm::style::{Color as CC, Stylize},
    rand::Rng,
};

/// Return the hexa notation commonly used for the web
pub fn to_hexa(rgb: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

fn print_color(c: Color) {
    let cc: CC = c.into();
    print!("{}", "██████".with(cc));
}

fn compare(rgb: Rgb) {
    print!(" {} ", to_hexa(rgb));
    print_color(rgb.into());
    let ansi = rgb.to_ansi();
    print_color(ansi.into());
    print!(" {}", ansi.code);
    println!();
}

fn main() {
    let mut rng = rand::thread_rng();
    println!(" Print some random RGB colors and the nearest ANSI color");
    println!("           rgb  ansi");
    for _ in 0..20 {
        let rgb = Rgb::new(rng.gen(), rng.gen(), rng.gen());
        compare(rgb);
    }
    println!();
}

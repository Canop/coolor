//! Run this with
//!     cargo run --example dark-to-light
use {
    coolor::AnsiColor,
    crossterm::style::{Color, Stylize},
};

fn print_color(ansi: AnsiColor) {
    print!("{}", "█".with(Color::AnsiValue(ansi.code)));
}

fn main() {
    println!(" Variations on ANSI colors:");
    println!(" ┌──────┬────────┬─────────────────┐");
    println!(" │ code │ normal │  dark to light  │");
    println!(" ├──────┼────────┼─────────────────┤");
    for code in 0..=255 {
        let ansi = AnsiColor::new(code);
        print!(" │  {:>3} │ ", code);
        for _ in 0..6 {
            print_color(ansi);
        }
        print!(" │ ");
        for i in -7..=7 {
            print_color(ansi.with_luminosity_change((i as f32)*0.1));
        }
        println!(" │");
    }
    println!(" └──────┴────────┴─────────────────┘");
}

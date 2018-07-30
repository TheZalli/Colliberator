extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

mod error;
mod color;
mod palette;

use palette::*;
use color::*;

fn main() {
    let pal = Palette::from_file("data/palette.txt").unwrap();

    for (name, set) in pal.iter_colorsets() {
        println!("Colorset `{}`:", name);
        for color in set.iter() {
            let color_name = pal.name_color(color).unwrap();
            let color_info = ColorInfo::new(color);

            let fmt_name = color.ansi_bgcolor(&format!("{:^20}", color_name)) + ":";

            println!("  Color {} {}", fmt_name, color_info);
        }
    }

    const LEN: usize = 64;
    let mut s = [SRGB24Color::default(); LEN];
    let mut l = [LinRGB24Color::default(); LEN];

    for (i, (s, l)) in s.iter_mut().zip(l.iter_mut()).enumerate() {
        let val = (i * (256/LEN)) as u8;
        *s = SRGB24Color::new(val, val, val);
        *l = LinRGB24Color::new(val, val, val);
    }

    print!("\nsRGB greyscale:   ");
    for col in s.iter() {
        print!("{}", col.ansi_bgcolor("_"));
    }

    print!("\nlinear greyscale: ");
    for col in l.iter() {
        print!("{}", col.ansi_bgcolor("_"));
    }
    println!();
}

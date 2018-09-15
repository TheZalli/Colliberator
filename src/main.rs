extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;
extern crate cgmath;

mod util;
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

            let fmt_name = ansi_bgcolor(color, &format!("{:^20}", color_name)) + ":";

            println!("  Color {} {}", fmt_name, color_info);
        }
    }

    const LEN: usize = 64;
    let mut s = [SRGB24Color::default(); LEN];
    let mut l = [LinRGB48Color::default(); LEN];

    for (i, (s, l)) in s.iter_mut().zip(l.iter_mut()).enumerate() {
        let (val8, val16) = ((i * 256/LEN) as u8, (i * 65535/LEN) as u16);

        *s = SRGB24Color::new(val8, val8, val8);
        *l = LinRGB48Color::new(val16, val16, val16);
    }

    print!("\nsRGB greyscale:   ");
    for col in s.iter() {
        print!("{}", ansi_bgcolor(*col, "_"));
    }

    print!("\nlinear greyscale: ");
    for col in l.iter() {
        print!("{}", ansi_bgcolor(col.to_float().encode().quantizate_u8(), "_"));
    }
    println!();
}

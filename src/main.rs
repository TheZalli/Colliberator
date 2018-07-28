extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

mod error;
mod color;
mod palette;

use palette::*;
use color::Color;

fn main() {
    let pal = Palette::from_file("data/palette.txt").unwrap();

    for (name, set) in pal.iter_colorsets() {
        println!("Colorset `{}`:", name);
        for color in set.iter() {
            let color_name = pal.name_color(color).unwrap();
            let color_info = ColorInfo::new(color);

            let fmt_name = color.ansi_escape_bgcolor(&format!("{:^20}", color_name)) + ":";

            println!("  Color {} {}", fmt_name, color_info);
        }
    }
}

extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

mod error;
mod color;
mod palette;

use palette::*;

fn main() {
    let pal = Palette::from_file("data/palette.txt").unwrap();
    //println!("{:#?}", pal);

    for (name, set) in pal.iter_colorsets() {
        println!("Colorset `{}`:", name);
        for color in set.iter() {
            let color_name = pal.name_color(color).unwrap();
            let color_info = ColorInfo::new(color);

            let fmt_name = String::new() + "'"+color_name+"':";

            println!("  Color {:<20} {}", fmt_name, color_info);
        }
    }
}
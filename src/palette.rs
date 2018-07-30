use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::iter::Iterator;

use regex::Regex;

use error::PaletteError;
use color::*;

lazy_static! {
    static ref SETNAME_RE: Regex =      Regex::new(r"^(.*?):").unwrap();
    static ref COLORLINE_RE: Regex =    Regex::new(r"^\*\s*([^#]+?)\s*#([0-9a-fA-F]{6})").unwrap();
}

#[derive(Debug)]
pub struct ColorSet {
    colors: Box<[SRGB24Color]>,
}

impl ColorSet {
    fn new(colors: Box<[SRGB24Color]>) -> Self { ColorSet{ colors } }
    pub fn iter<'a>(&'a self) -> ColorSetIter<'a> { ColorSetIter(self.colors.iter().cloned()) }
}

pub struct ColorSetIter<'a>(
    ::std::iter::Cloned<::std::slice::Iter<'a, SRGB24Color>>
);

impl<'a> Iterator for ColorSetIter<'a> {
    type Item = SRGB24Color;
    fn next(&mut self) -> Option<SRGB24Color> { self.0.next() }
}

#[derive(Debug)]
pub struct Palette {
    colors: BTreeMap<SRGB24Color, Box<str>>,
    colorsets: Vec<(Box<str>, ColorSet)>,
}

impl Palette {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, PaletteError> {
        let palettefile = File::open(path)?;
        Self::parse(BufReader::new(palettefile))
    }

    pub fn parse<T: BufRead>(input: T) -> Result<Self, PaletteError> {
        let mut colors = BTreeMap::new();
        let mut colorsets = Vec::new();

        for line in input.lines() {
            let line = line?;
            if let Some(capt) = SETNAME_RE.captures(&line) {
                // we got a color set name
                colorsets.push((capt[1].into(), Vec::new()));
            } else if let Some(capt) = COLORLINE_RE.captures(&line) {
                // we got a color
                let mut colname: Box<str> = capt[1].into();
                colname.make_ascii_lowercase();

                let rgb = unsafe { SRGB24Color::from_hex_unchecked(capt[2].into()) };

                if colorsets.is_empty() {
                    return Err(PaletteError::ColorWithoutSet { name: colname });
                }
                // record color set data
                colorsets.last_mut().unwrap().1.push(rgb);
                // record the color itself
                colors.insert(rgb, colname);
            } else {
                // empty line
                continue;
            }
        }
        let colorsets = colorsets.into_iter().map(|(k, v)| (k, ColorSet::new(v.into()))).collect();
        Ok(Palette { colors, colorsets })
    }

    /*pub fn get_colorset(&self, colorset_name: &str) -> Option<&ColorSet> {
        self.colorsets.get(colorset_name)
    }*/

    pub fn iter_colorsets<'a>(&'a self) -> ColorSetsIter<'a> {
        ColorSetsIter(self.colorsets.iter())
    }

    /// Returns the name of the given color, if it exists.
    pub fn name_color<T: Color>(&self, color: T) -> Option<&str> {
        Some(self.colors.get(&color.srgb24())?.as_ref())
    }
}

pub struct ColorSetsIter<'a>(
    ::std::slice::Iter<'a, (Box<str>, ColorSet)>
);

impl<'a> Iterator for ColorSetsIter<'a> {
    type Item = (&'a str, &'a ColorSet);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(name, set)| (name.as_ref(), set))
    }
}

#[derive(Debug)]
pub struct ColorInfo {
    srgb: SRGB24Color,
    lin_rgb: LinRGB24Color,
    hsv: HSVColor,
    shades_of: Vec<(BaseColor, f32)>,
}

impl ColorInfo {
    pub fn new<T: Color>(color: T) -> Self {
        ColorInfo {
            srgb: color.srgb24(),
            lin_rgb: color.lin_rgb24(),
            hsv: color.hsv(),
            shades_of: color.shades(),
        }
    }
}

impl fmt::Display for ColorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sRGB: ({}), HSV: ({}), lum: {:3.0}%, ",
               self.srgb, self.hsv, 100.0 * self.lin_rgb.relative_luminance())?;

        let fun = |f: &mut fmt::Formatter, color, _weight, sep| write!(f, " {}{}", color, sep);
        let (last, shades) = self.shades_of.split_last().unwrap();

        if let Some((last2nd, shades)) = shades.split_last() {
            write!(f, "is shades of")?;
            for (color, weight) in shades.iter() {
                fun(f, *color, *weight, ",")?;
            }
            fun(f, last2nd.0, last2nd.1, " and")?;
        } else {
            write!(f, "is a shade of")?;
        }
        fun(f, last.0, last.1, ".")
    }
}

use std::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, BTreeMap};
use std::iter::Iterator;

use regex::Regex;

use error::PaletteError;
use color::*;

lazy_static! {
    static ref SETNAME_RE: Regex = Regex::new(r"^(.*?):").unwrap();
    static ref COLORLINE_RE: Regex =
        Regex::new(r"^\*\s*([^#]+?)\s*#([0-9a-fA-F]{6})").unwrap();
}

#[derive(Debug)]
pub struct ColorSet {
    colors: Box<[ColorRGB]>,
}

impl ColorSet {
    fn new(colors: Box<[ColorRGB]>) -> Self {
        ColorSet{ colors }
    }

    pub fn iter<'a>(&'a self) -> ColorSetIter<'a> {
        ColorSetIter(self.colors.iter().cloned())
    }
}

pub struct ColorSetIter<'a>(
    ::std::iter::Cloned<::std::slice::Iter<'a, ColorRGB>>
);

impl<'a> Iterator for ColorSetIter<'a> {
    type Item = ColorRGB;

    fn next(&mut self) -> Option<ColorRGB> {
        self.0.next()
    }
}

#[derive(Debug)]
pub struct Palette {
    colors: BTreeMap<ColorRGB, Box<str>>,
    colorsets: HashMap<Box<str>, ColorSet>,
}

impl Palette {
    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<Self, PaletteError> {
        let palettefile = File::open(path)?;
        Self::parse(BufReader::new(palettefile))
    }

    pub fn parse<T: BufRead>(input: T) -> Result<Self, PaletteError> {
        let mut colors = BTreeMap::new();

        let mut colorsets = HashMap::new();
        let mut current_colorset_name: Box<str> = "".into();

        for line in input.lines() {
            let line = line?;
            if let Some(capt) = SETNAME_RE.captures(&line) {
                // we got a color set name

                current_colorset_name = capt[1].into();

            } else if let Some(capt) = COLORLINE_RE.captures(&line) {
                // we got a color

                let mut colname: Box<str> = capt[1].into();
                colname.make_ascii_lowercase();
                let colname: Box<str> = colname;

                let rgb = unsafe { ColorRGB::from_hex_unchecked(capt[2].into()) };

                if current_colorset_name.as_ref() == "" {
                    return Err(PaletteError::ColorWithoutSet { name: colname });
                }

                // record color set data
                colorsets.entry(current_colorset_name.clone()).or_insert(Vec::new()).push(rgb);

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
        Some(self.colors.get(&color.rgb())?.as_ref())
    }

}

pub struct ColorSetsIter<'a>(
    ::std::collections::hash_map::Iter<'a, Box<str>, ColorSet>
);

impl<'a> Iterator for ColorSetsIter<'a> {
    type Item = (&'a str, &'a ColorSet);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(name, set)| (name.as_ref(), set))
    }
}

#[derive(Debug)]
pub struct ColorInfo {
    pub rgb: ColorRGB,
    pub hsv: ColorHSV,
    pub shades_of: Vec<(f32, BaseColor)>,
}

impl ColorInfo {
    pub fn new<T: Color>(color: T) -> Self {
        ColorInfo {
            rgb: color.rgb(),
            hsv: color.hsv(),
            shades_of: color.shades(),
        }
    }
}

impl fmt::Display for ColorInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb: ({}), hsv: ({}), shades of [", self.rgb, self.hsv)?;

        for (weight, color) in self.shades_of.iter() {
            write!(f, " {} ({:.2}),", color, weight)?;
        }

        write!(f, "].")
    }
}

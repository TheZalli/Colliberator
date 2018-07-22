extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use regex::Regex;

use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, BTreeMap};

type Color = u32;

#[derive(Debug, Fail)]
enum PaletteError {
    #[fail(display = "Color `{}` declared without any color set!", name)]
    ColorWithoutSet {
        name: Box<str>
    },
    #[fail(display = "IO error: {}", inner)]
    IO {
        inner: std::io::Error,
    }
}

impl From<std::io::Error> for PaletteError {
    fn from(e: std::io::Error) -> Self {
        PaletteError::IO { inner: e }
    }
}

#[derive(Debug)]
struct Palette {
    colors: BTreeMap<Color, Box<str>>,
    colorsets: HashMap<Box<str>, Box<[Color]>>,
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

                let colcode = Color::from_str_radix(&capt[2], 16).unwrap();

                if current_colorset_name.as_ref() == "" {
                    return Err(PaletteError::ColorWithoutSet { name: colname });
                }

                // record color set data
                colorsets.entry(current_colorset_name.clone()).or_insert(Vec::new()).push(colcode);

                // record the color itself
                colors.insert(colcode, colname);
            } else {
                // empty line
                continue;
            }
        }

        let colorsets = colorsets.into_iter().map(|(k, v)| (k, v.into())).collect();

        Ok(Palette { colors, colorsets })
    }
}

lazy_static! {
    static ref SETNAME_RE: Regex = Regex::new(r"^(.*?):").unwrap();
    static ref COLORLINE_RE: Regex =
        Regex::new(r"^\*\s*([^#]+?)\s*#([0-9a-fA-F]{6})").unwrap();
}

fn main() {
    let pal = Palette::from_file("data/palette.txt");
    println!("{:#?}", pal);
}

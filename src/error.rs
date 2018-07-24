use std::io;

#[derive(Debug, Fail)]
pub enum PaletteError {
    #[fail(display = "Color `{}` declared without any color set!", name)]
    ColorWithoutSet {
        name: Box<str>
    },
    #[fail(display = "IO error: {}", inner)]
    IO { inner: io::Error }
}

impl From<io::Error> for PaletteError {
    fn from(e: io::Error) -> Self {
        PaletteError::IO { inner: e }
    }
}

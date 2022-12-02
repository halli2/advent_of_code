#![allow(dead_code)]
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    StdParse(core::num::ParseIntError),
    ArgParse(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::ArgParse(v) => v.fmt(f),
            Self::StdParse(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<core::num::ParseIntError> for Error {
    fn from(value: core::num::ParseIntError) -> Self {
        Self::StdParse(value)
    }
}

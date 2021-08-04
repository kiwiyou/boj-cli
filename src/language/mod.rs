use std::ffi::OsStr;

mod kotlin;
mod python;
mod rust;

pub struct Language {
    pub name: &'static str,
    pub ext: &'static str,
    pub compile: Option<Command>,
    pub execute: Command,
    pub version: &'static str,
}

pub struct Command {
    pub exec: &'static str,
    pub args: &'static [&'static str],
}

pub const LANGUAGES: &[Language] = &[
    rust::RUST_2015,
    rust::RUST_2018,
    python::PYTHON2,
    python::PYTHON3,
    kotlin::KOTLIN_JVM,
    kotlin::KOTLIN_NATIVE,
];

pub fn find_by_ext(ext: &OsStr) -> impl Iterator<Item = &'static Language> + '_ {
    LANGUAGES.iter().filter(move |l| l.ext == ext)
}

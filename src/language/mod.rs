use std::ffi::OsStr;

mod asm;
mod kotlin;
mod python;
mod rust;

pub struct Language {
    pub name: &'static str,
    pub ext: &'static str,
    pub compile: Option<&'static str>,
    pub execute: &'static str,
    pub version: &'static str,
}

pub const LANGUAGES: &[Language] = &[
    rust::RUST_2015,
    rust::RUST_2018,
    python::PYTHON2,
    python::PYTHON3,
    kotlin::KOTLIN_JVM,
    kotlin::KOTLIN_NATIVE,
    asm::ASSEMBLY_X32,
    asm::ASSEMBLY_X64,
];

pub fn find_by_ext(ext: &OsStr) -> impl Iterator<Item = &'static Language> + '_ {
    LANGUAGES.iter().filter(move |l| l.ext == ext)
}

use super::{Command, Language};

pub const RUST_2015: Language = Language {
    name: "Rust 2015",
    ext: "rs",
    compile: Some(Command {
        exec: "rustc",
        args: &[
            "--edition",
            "2015",
            "-O",
            "-o",
            "Main",
            "Main.rs",
        ],
    }),
    execute: Command {
        exec: "./Main",
        args: &[],
    },
    version: "rustc 1.52.1 (9bc8c42bb 2021-05-09)",
};
pub const RUST_2018: Language = Language {
    name: "Rust 2018",
    ext: "rs",
    compile: Some(Command {
        exec: "rustc",
        args: &[
            "--edition",
            "2018",
            "-O",
            "-o",
            "Main",
            "Main.rs",
        ],
    }),
    execute: Command {
        exec: "./Main",
        args: &[],
    },
    version: "rustc 1.52.1 (9bc8c42bb 2021-05-09)",
};

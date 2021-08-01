use super::{Command, Language};

pub const RUST_2015: Language = Language {
    name: "Rust 2015",
    short: "rs15",
    ext: "rs",
    compile: Some(Command {
        exec: "rustc",
        args: &[
            "--edition",
            "2015",
            "-O",
            "-o",
            "/tmp/solution",
            "solution.rs",
        ],
    }),
    execute: Command {
        exec: "/tmp/solution",
        args: &[],
    },
    version: "rustc 1.52.1 (9bc8c42bb 2021-05-09)",
};
pub const RUST_2018: Language = Language {
    name: "Rust 2018",
    short: "rs18",
    ext: "rs",
    compile: Some(Command {
        exec: "rustc",
        args: &[
            "--edition",
            "2018",
            "-O",
            "-o",
            "/tmp/solution",
            "solution.rs",
        ],
    }),
    execute: Command {
        exec: "/tmp/solution",
        args: &[],
    },
    version: "rustc 1.52.1 (9bc8c42bb 2021-05-09)",
};

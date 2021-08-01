use super::{Command, Language};

pub const PYTHON2: Language = Language {
    name: "Python 2",
    short: "py2",
    ext: "py",
    compile: Some(Command {
        exec: "python2",
        args: &[
            "-c",
            "import py_compile; py_compile.compile(r'Main.py')",
        ],
    }),
    execute: Command {
        exec: "python2",
        args: &["Main.py"],
    },
    version: "Python 2.7.18",
};
pub const PYTHON3: Language = Language {
    name: "Python 3",
    short: "py3",
    ext: "py",
    compile: Some(Command {
        exec: "python3",
        args: &[
            "-c",
            "import py_compile; py_compile.compile(r'Main.py')",
        ],
    }),
    execute: Command {
        exec: "python3",
        args: &["Main.py"],
    },
    version: "Python 3.9.5",
};

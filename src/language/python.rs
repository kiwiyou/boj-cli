use super::Language;

pub const PYTHON2: Language = Language {
    name: "Python 2",
    ext: "py",
    compile: Some(r#"python2 -c 'import py_compile; py_compile.compile(r"Main.py")'"#),
    execute: "python2 Main.py",
    version: "Python 2.7.18",
};
pub const PYTHON3: Language = Language {
    name: "Python 3",
    ext: "py",
    compile: Some(r#"python3 -c 'import py_compile; py_compile.compile(r"Main.py")'"#),
    execute: "python3 Main.py",
    version: "Python 3.9.5",
};

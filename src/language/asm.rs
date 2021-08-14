use super::Language;

pub const ASSEMBLY_X32: Language = Language {
    name: "Assembly (32bit)",
    ext: "asm",
    compile: Some("nasm -f elf32 -o Main.o Main.asm && gcc -m32 -o Main Main.o"),
    execute: "./Main",
    version: "NASM version 2.15.05",
};
pub const ASSEMBLY_X64: Language = Language {
    name: "Assmelby (64bit)",
    ext: "asm",
    compile: Some("nasm -f elf64 -o Main.o Main.asm && gcc -o Main Main.o"),
    execute: "./Main",
    version: "NASM version 2.15.05",
};

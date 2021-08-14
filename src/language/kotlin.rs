use super::Language;

pub const KOTLIN_JVM: Language = Language {
    name: "Kotlin (JVM)",
    ext: "kt",
    compile: Some("kotlinc-jvm -J-Xms1024m -J-Xmx1920m -J-Xss512m -include-runtime -d Main.jar Main.kt"),
    execute: "java -Xmx1024m -Xmx1920m -Xss512m -Dfile.encoding=UTF-8 -XX:+UseSerialGC -DONLINE_JUDGE=1 -DBOJ=1 -jar Main.jar",
    version: "kotlinc-jvm 1.5.0 (JRE 1.8.0_201-b09)",
};
pub const KOTLIN_NATIVE: Language = Language {
    name: "Kotlin (Native)",
    ext: "kt",
    compile: Some("kotlinc-native -o Main -opt Main.kt"),
    execute: "./Main",
    version: "kotlinc-native 1.5.0-743 (JRE 1.8.0_201-b09)",
};

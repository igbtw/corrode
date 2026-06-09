// File classification: code, config, docs extension detection.
// Centralised so both metrics and output can reuse the same rules.

/// Extensions treated as non-code — used to filter hotspots and warnings
// to avoid noise from generated or data files.
const NON_CODE_EXTS: &[&str] = &[
    "lock", "md", "txt", "json", "svg", "png", "jpg", "jpeg", "ico", "pdf", "gif",
];

// Lightweight filter for hotspot/warning logic; use classify_extension()
// when a three-way code/config/docs split is needed.
pub(crate) fn is_code_extension(ext: &str) -> bool {
    !NON_CODE_EXTS.contains(&ext)
}

pub fn classify_extension(ext: &str) -> &'static str {
    match ext {
        "rs" | "go" | "py" | "rb" | "js" | "ts" | "java" | "kt" | "swift" | "c" | "h"
        | "cpp" | "hpp" | "cs" | "scala" | "ex" | "exs" | "php" | "r" | "dart" | "lua"
        | "sh" | "bash" | "zsh" | "fish" | "pl" | "pm" => "code",
        "toml" | "json" | "yaml" | "yml" | "ini" | "cfg" | "conf" | "xml" | "lock"
        | "gradle" | "sbt" | "mk" | "cmake" => "config",
        "md" | "txt" | "rst" | "adoc" | "org" => "docs",
        _ => "other",
    }
}

pub fn language_name(ext: &str) -> &str {
    match ext {
        "rs" => "Rust",
        "toml" => "TOML",
        "md" => "Markdown",
        "json" => "JSON",
        "yaml" | "yml" => "YAML",
        "txt" => "Text",
        "lock" => "Lock",
        other => other,
    }
}

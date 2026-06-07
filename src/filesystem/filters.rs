// Skip-list constants and reusable filter predicates for directory traversal.

// Build artifacts, VCS data, vendored deps, generated output dirs.
pub const SKIP_DIRS: &[&str] = &[
    "target", ".git", "node_modules", ".cargo",
    "dist", "build", "out", "coverage",
    ".next", ".nuxt", "vendor", ".cache",
];

// Binary / cache / media formats that would fail or pollute UTF-8 stats.
pub const SKIP_EXTENSIONS: &[&str] = &[
    "sqlite", "sqlite-wal", "db", "cache",
    "bin", "exe", "dll", "so", "dylib",
    "map", "log", "pdf", "zip", "tar", "gz",
    "jpg", "png", "svg", "ico",
];

pub fn is_skipped_dir(name: &str) -> bool {
    SKIP_DIRS.contains(&name)
}

pub fn is_skipped_extension(ext: &str) -> bool {
    SKIP_EXTENSIONS.contains(&ext)
}

pub fn is_minified_file(name: &str) -> bool {
    name.ends_with(".min.js") || name.ends_with(".min.css")
}

use corrode::models::{FileEntry, ProjectType};

fn make_file(path: &str, contents: &str) -> FileEntry {
    FileEntry {
        path: path.to_string(),
        contents: contents.to_string(),
        extension: path.rsplit('.').next().unwrap_or("").to_string(),
        line_count: contents.lines().count(),
        size_bytes: contents.len() as u64,
    }
}

#[test]
fn extract_cargo_deps_simple() {
    let files = vec![make_file(
        "Cargo.toml",
        "[dependencies]\nclap = \"4\"\nserde = { version = \"1\" }\n",
    )];
    let deps = corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Rust);
    assert!(deps.contains(&"clap".to_string()));
    assert!(deps.contains(&"serde".to_string()));
}

#[test]
fn extract_cargo_deps_ignores_next_section() {
    let files = vec![make_file(
        "Cargo.toml",
        "[dependencies]\nclap = \"4\"\n[build-dependencies]\ncc = \"1\"\n",
    )];
    let deps = corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Rust);
    assert!(deps.contains(&"clap".to_string()));
    assert!(!deps.contains(&"cc".to_string()));
}

#[test]
fn extract_cargo_deps_skips_comments() {
    let files = vec![make_file(
        "Cargo.toml",
        "[dependencies]\n# this is a comment\nclap = \"4\"\n",
    )];
    let deps = corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Rust);
    assert_eq!(deps, vec!["clap".to_string()]);
}

#[test]
fn extract_cargo_deps_empty_when_no_manifest() {
    let files = vec![make_file("src/main.rs", "fn main() {}")];
    let deps = corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Rust);
    assert!(deps.is_empty());
}

#[test]
fn extract_deps_noop_for_unknown_project() {
    let files = vec![make_file("package.json", "{\"dependencies\": {\"react\": \"^18\"}}")];
    let deps =
        corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Unknown);
    assert!(deps.is_empty());
}

#[test]
fn extract_deps_noop_for_node() {
    // Node dep parsing is not implemented yet; should return empty.
    let files = vec![make_file("package.json", "{\"dependencies\": {\"react\": \"^18\"}}")];
    let deps = corrode::analysis::dependencies::detect_dependencies(&files, &ProjectType::Node);
    assert!(deps.is_empty());
}

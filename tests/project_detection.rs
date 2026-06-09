use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn with_temp_dir(test: impl FnOnce(&Path)) {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir().join(format!("corrode_pd_{}_{}", std::process::id(), id));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    test(&base);
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn detect_rust_from_cargo_toml() {
    with_temp_dir(|root| {
        fs::write(root.join("Cargo.toml"), "[package]\nname = \"test\"\n").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Rust
        );
    });
}

#[test]
fn detect_node_from_package_json() {
    with_temp_dir(|root| {
        fs::write(root.join("package.json"), "{}").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Node
        );
    });
}

#[test]
fn detect_go_from_go_mod() {
    with_temp_dir(|root| {
        fs::write(root.join("go.mod"), "module test").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Go
        );
    });
}

#[test]
fn detect_python_from_pyproject_toml() {
    with_temp_dir(|root| {
        fs::write(root.join("pyproject.toml"), "").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Python
        );
    });
}

#[test]
fn detect_python_from_setup_py() {
    with_temp_dir(|root| {
        fs::write(root.join("setup.py"), "").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Python
        );
    });
}

#[test]
fn detect_ruby_from_gemfile() {
    with_temp_dir(|root| {
        fs::write(root.join("Gemfile"), "").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Ruby
        );
    });
}

#[test]
fn detect_unknown_for_empty_dir() {
    with_temp_dir(|root| {
        assert_eq!(
            corrode::analysis::project::detect_project_type(&root.to_string_lossy()),
            corrode::models::ProjectType::Unknown
        );
    });
}

#[test]
fn detect_unknown_for_single_file() {
    with_temp_dir(|root| {
        let f = root.join("data.txt");
        fs::write(&f, "data").unwrap();
        assert_eq!(
            corrode::analysis::project::detect_project_type(&f.to_string_lossy()),
            corrode::models::ProjectType::Unknown
        );
    });
}

#[test]
fn detect_rust_entry_point() {
    let files = vec!["src/main.rs".to_string(), "src/lib.rs".to_string()];
    let ep = corrode::analysis::project::detect_entry_point(
        &files,
        &corrode::models::ProjectType::Rust,
    );
    assert_eq!(ep, Some("src/main.rs".to_string()));
}

#[test]
fn detect_entry_point_respects_priority_order() {
    let files = vec!["src/lib.rs".to_string()];
    let ep = corrode::analysis::project::detect_entry_point(
        &files,
        &corrode::models::ProjectType::Rust,
    );
    assert_eq!(ep, Some("src/lib.rs".to_string()));
}

#[test]
fn detect_no_entry_point_for_unknown_type() {
    let files = vec!["main.rs".to_string()];
    let ep = corrode::analysis::project::detect_entry_point(
        &files,
        &corrode::models::ProjectType::Unknown,
    );
    assert_eq!(ep, None);
}

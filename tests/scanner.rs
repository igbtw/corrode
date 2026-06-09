use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn with_temp_dir(test: impl FnOnce(&Path)) {
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let base = std::env::temp_dir().join(format!("corrode_test_{}_{}", std::process::id(), id));
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    test(&base);
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn scan_detects_source_files() {
    with_temp_dir(|root| {
        fs::write(root.join("main.rs"), "fn main() {}").unwrap();
        fs::write(root.join("lib.rs"), "pub fn foo() {}").unwrap();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join("src/app.rs"), "mod app;").unwrap();

        let files = corrode::filesystem::scanner::scan_directory(
            &root.to_string_lossy(),
        );
        let names: Vec<&str> = files
            .iter()
            .map(|f| Path::new(f).file_name().unwrap().to_str().unwrap())
            .collect();
        assert!(names.contains(&"main.rs"));
        assert!(names.contains(&"lib.rs"));
        assert!(names.contains(&"app.rs"));
    });
}

#[test]
fn scan_skips_target_dir() {
    with_temp_dir(|root| {
        fs::write(root.join("main.rs"), "fn main() {}").unwrap();
        fs::create_dir_all(root.join("target/debug")).unwrap();
        fs::write(root.join("target/debug/foo"), "").unwrap();

        let files = corrode::filesystem::scanner::scan_directory(
            &root.to_string_lossy(),
        );
        let names: Vec<&str> = files
            .iter()
            .map(|f| Path::new(f).file_name().unwrap().to_str().unwrap())
            .collect();
        assert_eq!(names, vec!["main.rs"]);
    });
}

#[test]
fn count_directories_matches_expected() {
    with_temp_dir(|root| {
        fs::write(root.join("main.rs"), "fn main() {}").unwrap();
        fs::create_dir_all(root.join("src/utils")).unwrap();
        fs::write(root.join("src/utils/helper.rs"), "// helper").unwrap();
        fs::create_dir_all(root.join("target")).unwrap();

        let count = corrode::filesystem::scanner::count_directories(
            &root.to_string_lossy(),
        );
        // root/ + src/ + src/utils/ = 2 non-root dirs
        assert_eq!(count, 2);
    });
}

#[test]
fn scan_empty_dir_returns_empty_list() {
    with_temp_dir(|root| {
        let files = corrode::filesystem::scanner::scan_directory(
            &root.to_string_lossy(),
        );
        assert!(files.is_empty());
    });
}

#[test]
fn scan_handles_single_file() {
    with_temp_dir(|root| {
        let file_path = root.join("notes.txt");
        fs::write(&file_path, "hello").unwrap();

        let files = corrode::filesystem::scanner::scan_directory(
            &root.to_string_lossy(),
        );
        // .txt is not in SKIP_EXTENSIONS, so it should appear.
        assert!(!files.is_empty());
        assert!(files[0].ends_with("notes.txt"));
    });
}

#[test]
fn count_directories_skips_ignored_dirs() {
    with_temp_dir(|root| {
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("node_modules/pkg")).unwrap();
        fs::create_dir_all(root.join("target/debug")).unwrap();

        let count = corrode::filesystem::scanner::count_directories(
            &root.to_string_lossy(),
        );
        // Only src/ should count; node_modules/ and target/ are skipped.
        assert_eq!(count, 1);
    });
}

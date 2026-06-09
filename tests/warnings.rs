use corrode::models::FileEntry;

fn file(path: &str, ext: &str, lines: usize) -> FileEntry {
    FileEntry {
        path: path.to_string(),
        contents: String::new(),
        extension: ext.to_string(),
        line_count: lines,
        size_bytes: (lines * 10) as u64,
    }
}

#[test]
fn warn_when_largest_file_exceeds_5_percent() {
    let files = vec![
        file("big.rs", "rs", 600),
        file("small.rs", "rs", 10),
        file("tiny.rs", "rs", 5),
    ];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 3);
    let has_large_file_warning = warnings.iter().any(|w| w.contains("Largest code file"));
    assert!(has_large_file_warning);
}

#[test]
fn no_warn_when_files_are_small() {
    // 20 equal-sized files so largest is exactly 5% of code LOC (threshold).
    let files: Vec<FileEntry> = (0..20).map(|i| file(&format!("f{}.rs", i), "rs", 10)).collect();
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 3);
    let large_file_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("Largest code file")).collect();
    assert!(large_file_warnings.is_empty());
}

#[test]
fn warn_when_markdown_exceeds_20_percent() {
    let md_file = file("readme.md", "md", 10);
    let code_files: Vec<FileEntry> = (0..5).map(|i| file(&format!("f{}.rs", i), "rs", 10)).collect();
    let mut files = vec![md_file];
    files.extend(code_files);
    // 1 md out of 6 files = 16.6% — below 20% threshold
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 3);
    let md_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("Markdown")).collect();
    assert!(md_warnings.is_empty());

    // Add enough .md to exceed 20%
    let more_md: Vec<FileEntry> = (0..5).map(|i| file(&format!("doc{}.md", i), "md", 5)).collect();
    files.extend(more_md);
    // 6 md out of 11 files = 54.5%
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 3);
    let md_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("Markdown")).collect();
    assert!(!md_warnings.is_empty());
}

#[test]
fn warn_when_depth_exceeds_8() {
    let files = vec![file("a.rs", "rs", 10)];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 9);
    let has_depth_warning = warnings.iter().any(|w| w.contains("depth exceeds 8"));
    assert!(has_depth_warning);
}

#[test]
fn no_depth_warning_when_shallow() {
    let files = vec![file("a.rs", "rs", 10)];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 4);
    let depth_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("depth exceeds")).collect();
    assert!(depth_warnings.is_empty());
}

#[test]
fn warn_when_no_readme() {
    let files = vec![file("main.rs", "rs", 10)];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 2);
    let has_readme_warning = warnings.iter().any(|w| w.contains("README.md"));
    assert!(has_readme_warning);
}

#[test]
fn no_readme_warning_when_readme_exists() {
    let files = vec![
        file("main.rs", "rs", 10),
        file("README.md", "md", 5),
    ];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 2);
    let readme_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("README.md")).collect();
    assert!(readme_warnings.is_empty());
}

#[test]
fn warn_when_no_tests_directory() {
    let files = vec![file("main.rs", "rs", 10)];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 2);
    let has_test_warning = warnings.iter().any(|w| w.contains("tests/"));
    assert!(has_test_warning);
}

#[test]
fn no_test_warning_when_tests_dir_exists() {
    let files = vec![
        file("src/main.rs", "rs", 10),
        file("tests/test_main.rs", "rs", 20),
    ];
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 3);
    let test_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("tests/")).collect();
    assert!(test_warnings.is_empty());
}

#[test]
fn no_warnings_for_empty_files() {
    let warnings = corrode::analysis::warnings::compute_warnings(&[], 0);
    assert!(warnings.is_empty());
}

#[test]
fn warn_when_json_exceeds_100() {
    let files: Vec<FileEntry> = (0..101).map(|i| file(&format!("data{}.json", i), "json", 10)).collect();
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 2);
    let json_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("JSON")).collect();
    assert!(!json_warnings.is_empty());
}

#[test]
fn no_json_warning_when_under_threshold() {
    let files: Vec<FileEntry> = (0..50).map(|i| file(&format!("data{}.json", i), "json", 10)).collect();
    let warnings = corrode::analysis::warnings::compute_warnings(&files, 2);
    let json_warnings: Vec<_> = warnings.iter().filter(|w| w.contains("JSON")).collect();
    assert!(json_warnings.is_empty());
}

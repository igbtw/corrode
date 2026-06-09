use corrode::models::FileEntry;

fn file(path: &str, ext: &str, lines: usize, bytes: u64) -> FileEntry {
    FileEntry {
        path: path.to_string(),
        contents: String::new(),
        extension: ext.to_string(),
        line_count: lines,
        size_bytes: bytes,
    }
}

#[test]
fn language_map_groups_by_extension() {
    let files = vec![
        file("a.rs", "rs", 10, 100),
        file("b.rs", "rs", 20, 200),
        file("c.py", "py", 30, 300),
    ];
    let map = corrode::analysis::metrics::build_language_map(&files);
    assert_eq!(map.get("rs"), Some(&2));
    assert_eq!(map.get("py"), Some(&1));
}

#[test]
fn total_lines_sums_all_lines() {
    let files = vec![
        file("a.rs", "rs", 10, 100),
        file("b.rs", "rs", 20, 200),
    ];
    assert_eq!(corrode::analysis::metrics::calculate_total_lines(&files), 30);
}

#[test]
fn depth_map_computes_relative_depths() {
    let files = vec![
        file("/fake/src/main.rs", "rs", 10, 100),
        file("/fake/src/lib.rs", "rs", 20, 200),
        file("/fake/build.rs", "rs", 5, 50),
    ];
    let map = corrode::analysis::metrics::compute_depth_map("/fake", &files);
    // build.rs is at depth 0 (strip_prefix gives "build.rs", components=1, saturating_sub(1)=0)
    // src/main.rs and src/lib.rs are at depth 1
    assert_eq!(map.iter().find(|(d, _)| *d == 0).map(|(_, c)| c), Some(&1));
    assert_eq!(map.iter().find(|(d, _)| *d == 1).map(|(_, c)| c), Some(&2));
}

#[test]
fn size_distribution_buckets_correctly() {
    let files = vec![
        file("noise.rs", "rs", 3, 30),
        file("small.rs", "rs", 50, 500),
        file("medium.rs", "rs", 200, 2000),
        file("large.rs", "rs", 800, 8000),
    ];
    let dist = corrode::analysis::metrics::compute_size_distribution(&files);
    assert_eq!(dist.noise, 1);
    assert_eq!(dist.small, 1);
    assert_eq!(dist.medium, 1);
    assert_eq!(dist.large, 1);
}

#[test]
fn code_metrics_splits_by_category() {
    let files = vec![
        file("main.rs", "rs", 50, 500),
        file("config.toml", "toml", 10, 100),
        file("readme.md", "md", 5, 50),
    ];
    let cm = corrode::analysis::metrics::compute_code_metrics(&files);
    assert_eq!(cm.code_files, 1);
    assert_eq!(cm.code_lines, 50);
    assert_eq!(cm.config_files, 1);
    assert_eq!(cm.config_lines, 10);
    assert_eq!(cm.docs_files, 1);
    assert_eq!(cm.docs_lines, 5);
}

#[test]
fn top_code_files_returns_n_largest() {
    let files = vec![
        file("small.rs", "rs", 5, 50),
        file("medium.rs", "rs", 50, 500),
        file("large.rs", "rs", 500, 5000),
    ];
    let top = corrode::analysis::metrics::top_code_files(&files, 2);
    assert_eq!(top.len(), 2);
    assert!(top[0].path.contains("large"));
    assert!(top[1].path.contains("medium"));
}

#[test]
fn top_code_files_excludes_non_code() {
    let files = vec![
        file("data.json", "json", 100, 1000),
        file("script.rs", "rs", 10, 100),
    ];
    let top = corrode::analysis::metrics::top_code_files(&files, 5);
    assert_eq!(top.len(), 1);
}

#[test]
fn compute_complexity_returns_low_for_small_project() {
    use corrode::models::SizeDistribution;
    let sd = SizeDistribution { noise: 0, small: 5, medium: 0, large: 0 };
    let result = corrode::analysis::complexity::compute_complexity(
        5, 1, 2, &[], &sd, 0.02,
    );
    assert_eq!(result.rating, "Low");
    assert!(result.score <= 20);
}

#[test]
fn compute_complexity_returns_extreme_for_large_project() {
    use corrode::models::SizeDistribution;
    use corrode::models::Hotspot;
    let sd = SizeDistribution { noise: 0, small: 0, medium: 0, large: 200 };
    let hotspots = vec![Hotspot {
        path: "/".into(),
        total_lines: 100_000,
        percentage: 100.0,
    }];
    let result = corrode::analysis::complexity::compute_complexity(
        40_000, 9, 100, &hotspots, &sd, 1.0,
    );
    assert_eq!(result.rating, "Extreme");
    assert_eq!(result.score, 100);
}

#[test]
fn compute_architecture_handles_empty() {
    let result = corrode::analysis::architecture::compute_architecture(&[], 0);
    assert_eq!(result.max_depth, 0);
    assert_eq!(result.avg_loc_per_file, 0.0);
}

#[test]
fn compute_hotspots_groups_by_directory() {
    let files = vec![
        file("/fake/src/main.rs", "rs", 100, 1000),
        file("/fake/src/lib.rs", "rs", 200, 2000),
        file("/fake/tests/test.rs", "rs", 50, 500),
    ];
    let hotspots = corrode::analysis::hotspots::compute_hotspots(&files, "/fake");
    assert!(!hotspots.is_empty());
    // src/ should be the top hotspot with 300 of 350 LOC.
    let src = hotspots.iter().find(|h| h.path == "src/").unwrap();
    assert_eq!(src.total_lines, 300);
}

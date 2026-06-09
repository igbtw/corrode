// Hotspot computation: groups code files by parent directory.
// Uses LOC share rather than file count so a dir with 100 small files
// does not appear more concentrated than one with 10 large files.

use std::collections::HashMap;
use std::path::Path;

use crate::analysis::classification::is_code_extension;
use crate::models::{FileEntry, Hotspot};

pub fn compute_hotspots(files: &[FileEntry], project_root: &str) -> Vec<Hotspot> {
    let code_files: Vec<_> = files
        .iter()
        .filter(|f| is_code_extension(&f.extension))
        .collect();
    if code_files.is_empty() {
        return Vec::new();
    }

    let code_total: usize = code_files.iter().map(|f| f.line_count).sum();
    let root = Path::new(project_root);

    let mut groups: HashMap<String, usize> = HashMap::new();
    for file in &code_files {
        let parent = Path::new(&file.path).parent().map(|p| {
            p.strip_prefix(root)
                .unwrap_or(p)
                .display()
                .to_string()
        });

        let key = match parent {
            Some(ref p) if !p.is_empty() && p != "." && p != project_root => {
                format!("{}/", p)
            }
            _ => String::from("(root)"),
        };

        *groups.entry(key).or_insert(0) += file.line_count;
    }

    let mut sorted: Vec<Hotspot> = groups
        .into_iter()
        .map(|(path, loc)| Hotspot {
            percentage: loc as f64 / code_total as f64 * 100.0,
            path,
            total_lines: loc,
        })
        .collect();

    sorted.sort_by(|a, b| b.total_lines.cmp(&a.total_lines));

    // Top 5 directories; aggregate remaining into "other" for concise output.
    let mut hotspots: Vec<Hotspot> = Vec::with_capacity(6);
    let mut other_lines = 0usize;

    for (i, spot) in sorted.into_iter().enumerate() {
        if i < 5 {
            hotspots.push(spot);
        } else {
            other_lines += spot.total_lines;
        }
    }

    if !hotspots.is_empty() && other_lines > 0 {
        hotspots.push(Hotspot {
            path: String::from("other"),
            total_lines: other_lines,
            percentage: other_lines as f64 / code_total as f64 * 100.0,
        });
    } else if hotspots.is_empty() && other_lines > 0 {
        hotspots.push(Hotspot {
            path: String::from("other"),
            total_lines: other_lines,
            percentage: 100.0,
        });
    }

    hotspots
}

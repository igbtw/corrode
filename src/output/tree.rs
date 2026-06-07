// Prints a directory tree (├── └── format) respecting the same
// skip rules as scan_directory.

use crate::filesystem::scanner::{SKIP_DIRS, SKIP_EXTENSIONS};
use walkdir::WalkDir;

pub fn print_tree(path: &str) {
    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.depth() == 0
                || e.file_type().is_dir()
                || !SKIP_EXTENSIONS.contains(
                    &e.path().extension().and_then(|e| e.to_str()).unwrap_or(""),
                )
        })
        .collect();

    println!(".");
    let mut stack: Vec<(usize, bool)> = Vec::new();

    for i in 1..entries.len() {
        let entry = &entries[i];
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy();

        while stack.last().map(|(d, _)| *d >= depth).unwrap_or(false) {
            stack.pop();
        }

        // Scan ahead for the next entry at this depth or shallower — that
        // determines whether this entry is the last sibling at its level.
        // For directories this automatically skips all children.
        let next_sibling = entries[i + 1..]
            .iter()
            .position(|e| e.depth() == depth)
            .map(|pos| i + 1 + pos)
            .unwrap_or(entries.len());
        let is_last = next_sibling >= entries.len();

        let mut prefix = String::new();
        for (_, last) in &stack {
            prefix.push_str(if *last { "    " } else { "│   " });
        }

        let display = if entry.file_type().is_dir() {
            format!("{}/", name)
        } else {
            name.to_string()
        };

        println!("{}{}{}", prefix, if is_last { "└── " } else { "├── " }, display);

        if entry.file_type().is_dir() {
            stack.push((depth, is_last));
        }
    }
}

// Filesystem module — directory walking, file discovery, project
// detection.  Uses walkdir internally for recursive traversal.
//
// Public API:
//   scanner::scan_directory()
//   scanner::count_directories()
//   scanner::detect_project_type()

pub mod scanner;

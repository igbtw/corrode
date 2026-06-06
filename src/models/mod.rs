// Data models — types that represent a codebase's structure
//
// TODO: define structs like Project, Module, Function, Dependency,
// Issue, etc. to represent the output of code analysis.

pub struct FileEntry {
    pub path: String,
    pub contents: String,
}

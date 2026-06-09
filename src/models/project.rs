use std::fmt;
use std::time::Duration;

use serde::Serialize;

mod duration_serde {
    use serde::ser::Serializer;
    use std::time::Duration;

    pub fn serialize<S>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(dur.as_secs_f64())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ProjectType {
    Rust,
    Node,
    Go,
    Python,
    Ruby,
    Unknown,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Rust => write!(f, "Rust"),
            ProjectType::Node => write!(f, "Node"),
            ProjectType::Go => write!(f, "Go"),
            ProjectType::Python => write!(f, "Python"),
            ProjectType::Ruby => write!(f, "Ruby"),
            ProjectType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize)]
pub struct ProjectInfo {
    pub project_type: ProjectType,
    pub entry_point: Option<String>,
    pub project_root: String,
    #[serde(with = "duration_serde")]
    pub duration: Duration,
}

use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pending,
    Extracted,
    Error,
    Skipped,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileState {
    pub path: String,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compatibility: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunState {
    pub id: String,
    pub files: Vec<FileState>,
}

impl RunState {
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json)
    }
    pub fn load(path: &Path) -> Option<Self> {
        let data = std::fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }
}

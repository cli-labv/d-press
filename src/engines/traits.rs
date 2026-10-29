use std::path::Path;

/// Info mínima del archivo detectado
#[derive(Debug, Clone)]
pub struct ArchiveInfo {
    pub format: &'static str,
    pub encrypted: bool,
    pub multi_volume: bool,
    pub solid: bool,
    pub total_unpacked: u64,
}

#[derive(Debug, Default, Clone)]
pub struct ExtractOutcome {
    pub notices: Vec<String>,
}

impl ExtractOutcome {
    pub fn special(notices: Vec<String>) -> Self {
        Self { notices }
    }
}

pub trait Engine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String>;
    fn extract(&self, path: &Path, dest: &Path) -> Result<ExtractOutcome, String>;
}

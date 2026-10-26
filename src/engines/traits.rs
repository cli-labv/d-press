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

pub trait Engine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String>;
    fn extract(&self, path: &Path, dest: &Path) -> Result<(), String>;
}

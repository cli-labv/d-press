use jwalk::WalkDir;
use std::path::Path;

/// Recorre input y devuelve lista de rutas a archivos posiblemente comprimidos
pub fn find_archives(root: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            out.push(entry.path());
        }
    }
    out
}

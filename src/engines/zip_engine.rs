use crate::core::portable::{PathFix, fix_relative_path};
use crate::engines::external;
use crate::engines::traits::{ArchiveInfo, Engine, ExtractOutcome};
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;

pub struct ZipEngine;

impl Engine for ZipEngine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String> {
        // Basic probe: check signature
        let data = fs::read(path).map_err(|e| e.to_string())?;
        if !data.starts_with(b"PK\x03\x04")
            && !data.starts_with(b"PK\x05\x06")
            && !data.starts_with(b"PK\x07\x08")
        {
            return Err("not zip".into());
        }
        Ok(ArchiveInfo {
            format: "zip",
            encrypted: false,
            multi_volume: false,
            solid: false,
            total_unpacked: 0,
        })
    }

    fn extract(&self, path: &Path, dest: &Path) -> Result<ExtractOutcome, String> {
        match self.extract_native(path, dest) {
            Ok(outcome) => Ok(outcome),
            Err(error) if error.starts_with("ruta insegura") => Err(error),
            Err(native_error) => match external::for_zip() {
                Some(tool) => match external::extract(tool, path, dest) {
                    Ok(()) => Ok(ExtractOutcome::special(vec![format!(
                        "el motor ZIP nativo falló ({native_error}); se usó {} como respaldo",
                        tool.name()
                    )])),
                    Err(fallback_error) => Err(format!(
                        "{native_error}; el respaldo {} también falló: {fallback_error}",
                        tool.name()
                    )),
                },
                None => Err(format!(
                    "{native_error}; no hay motor externo de respaldo disponible"
                )),
            },
        }
    }
}

impl ZipEngine {
    fn extract_native(&self, path: &Path, dest: &Path) -> Result<ExtractOutcome, String> {
        fs::create_dir_all(dest).map_err(|e| e.to_string())?;
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

        let mut fixes: Vec<PathFix> = Vec::new();
        for i in 0..archive.len() {
            let entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = entry.name().map_err(|e| e.to_string())?;
            let fixed = fix_relative_path(&name)?;
            if fixed.changed() {
                fixes.push(fixed);
            }
        }

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = entry.name().map_err(|e| e.to_string())?;
            let fixed = fix_relative_path(&name)?;
            if fixed.value.as_os_str().is_empty() {
                continue;
            }
            let out_path = dest.join(&fixed.value);
            if name.ends_with('/') {
                fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
                continue;
            }
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut outfile = File::create(&out_path).map_err(|e| e.to_string())?;
            copy(&mut entry, &mut outfile).map_err(|e| e.to_string())?;
        }

        Ok(ExtractOutcome::special(name_notices(&fixes)))
    }
}

fn name_notices(fixes: &[PathFix]) -> Vec<String> {
    if fixes.is_empty() {
        return Vec::new();
    }
    let mut notices = vec![format!(
        "{} nombre(s) de entrada ajustados para portabilidad (puntos/espacios finales, caracteres no válidos o nombres reservados)",
        fixes.len()
    )];
    for fix in fixes.iter().take(5) {
        notices.push(format!(
            "«{}» → «{}» ({})",
            fix.original,
            fix.value.display(),
            fix.reason()
        ));
    }
    if fixes.len() > 5 {
        notices.push(format!("y {} más", fixes.len() - 5));
    }
    notices
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn sanitizes_entry_names_and_reports_special_mode() {
        let base = std::env::temp_dir().join(format!(
            "dpress-zip-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&base).unwrap();
        let archive_path = base.join("test....zip");

        let file = File::create(&archive_path).unwrap();
        let mut writer = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default();
        writer.start_file("sub.../page.webp.", options).unwrap();
        writer.write_all(b"data").unwrap();
        writer.finish().unwrap();

        let dest = base.join("out");
        let outcome = ZipEngine.extract(&archive_path, &dest).unwrap();

        assert!(dest.join("sub/page.webp").exists());
        assert!(!outcome.notices.is_empty());

        fs::remove_dir_all(&base).unwrap();
    }
}

use crate::core::portable::{PathFix, fix_relative_path};
use crate::engines::external;
use crate::engines::traits::{ArchiveInfo, Engine, ExtractOutcome};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{BufWriter, Error as IoError, ErrorKind, Write};
use std::path::Path;

pub struct RarEngine;

impl Engine for RarEngine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String> {
        // Detect family via rars
        let data = fs::read(path).map_err(|e| e.to_string())?;
        let sig = rars::find_archive_start(&data, usize::MAX).ok_or("no rar signature")?;
        let _family =
            rars::detect_archive_family(&data[sig.offset as usize..]).ok_or("unknown family")?;
        let encrypted = false; // placeholder
        let solid = false; // placeholder
        Ok(ArchiveInfo {
            format: "rar",
            encrypted,
            multi_volume: false,
            solid,
            total_unpacked: 0,
        })
    }

    fn extract(&self, path: &Path, dest: &Path) -> Result<ExtractOutcome, String> {
        match self.extract_native(path, dest) {
            Ok(outcome) => Ok(outcome),
            Err(error) if error.contains("ruta insegura") => Err(error),
            Err(native_error) => match external::for_rar() {
                Some(tool) => match external::extract(tool, path, dest) {
                    Ok(()) => Ok(ExtractOutcome::special(vec![format!(
                        "el motor RAR nativo falló ({native_error}); se usó {} como respaldo",
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

impl RarEngine {
    fn extract_native(&self, path: &Path, dest: &Path) -> Result<ExtractOutcome, String> {
        fs::create_dir_all(dest).map_err(|e| e.to_string())?;

        if let Ok(archive) = rars::rar50::Archive::parse_path(path) {
            let fixes = RefCell::new(Vec::new());
            let options = rars::ArchiveReadOptions::default();
            archive
                .extract_to(options, |meta| {
                    write_entry(&rars::rar50_meta(meta), dest, &fixes)
                })
                .map_err(|e| format!("{:?}", e))?;
            return Ok(ExtractOutcome::special(name_notices(&fixes.into_inner())));
        }

        if let Ok(archive) = rars::rar15_40::Archive::parse_path(path) {
            let fixes = RefCell::new(Vec::new());
            let options = rars::ArchiveReadOptions::default();
            archive
                .extract_to(options, |meta| write_entry(&rar15_meta(meta), dest, &fixes))
                .map_err(|e| format!("{:?}", e))?;
            return Ok(ExtractOutcome::special(name_notices(&fixes.into_inner())));
        }

        Err("No RAR parser matched".into())
    }
}

fn rar15_meta(meta: &rars::rar15_40::ExtractedEntryMeta) -> rars::ExtractedEntryMeta {
    rars::ExtractedEntryMeta::new(
        meta.name.clone(),
        meta.file_time,
        meta.attr as u64,
        meta.is_directory,
    )
}

fn write_entry(
    meta: &rars::ExtractedEntryMeta,
    dest: &Path,
    fixes: &RefCell<Vec<PathFix>>,
) -> rars::Result<Box<dyn Write>> {
    let raw = String::from_utf8_lossy(&meta.name);
    let fixed = fix_relative_path(&raw).map_err(|e| {
        rars::Error::from(IoError::new(
            ErrorKind::InvalidInput,
            format!("ruta insegura: {e}"),
        ))
    })?;

    if fixed.value.as_os_str().is_empty() {
        return Ok(Box::new(std::io::sink()));
    }

    let out_path = dest.join(&fixed.value);
    if meta.is_directory || raw.ends_with('/') {
        fs::create_dir_all(&out_path)
            .map_err(|e| rars::Error::from(IoError::new(e.kind(), e.to_string())))?;
        return Ok(Box::new(std::io::sink()));
    }

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| rars::Error::from(IoError::new(e.kind(), e.to_string())))?;
    }
    let file = File::create(&out_path)
        .map_err(|e| rars::Error::from(IoError::new(e.kind(), e.to_string())))?;

    if fixed.changed() {
        fixes.borrow_mut().push(fixed);
    }

    Ok(Box::new(BufWriter::new(file)))
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

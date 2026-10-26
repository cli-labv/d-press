use crate::engines::traits::{Engine, ArchiveInfo};
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::path::Path;

pub struct RarEngine;

impl Engine for RarEngine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String> {
        // Detect family via rars
        let data = fs::read(path).map_err(|e| e.to_string())?;
        let sig = rars::find_archive_start(&data, usize::MAX).ok_or("no rar signature")?;
        let family = rars::detect_archive_family(&data[sig.offset as usize..]).ok_or("unknown family")?;
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

    fn extract(&self, path: &Path, dest: &Path) -> Result<(), String> {
        fs::create_dir_all(dest).map_err(|e| e.to_string())?;

        // Try RAR5 first
        if let Ok(archive) = rars::rar50::Archive::parse_path(path) {
            let options = rars::ArchiveReadOptions::default();
            let dest_clone = dest.to_path_buf();
            archive.extract_to(options, move |meta| {
                let name = String::from_utf8_lossy(&meta.name);
                let out_path = dest_clone.join(name.as_ref());
                if let Some(parent) = out_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let file = File::create(&out_path).unwrap();
                Ok(Box::new(BufWriter::new(file)) as Box<dyn Write>)
            }).map_err(|e| format!("{:?}", e))?;
            return Ok(());
        }

        // Try RAR 1.5-4
        if let Ok(archive) = rars::rar15_40::Archive::parse_path(path) {
            let options = rars::ArchiveReadOptions::default();
            let dest_clone = dest.to_path_buf();
            archive.extract_to(options, move |meta| {
                let name = String::from_utf8_lossy(&meta.name);
                let out_path = dest_clone.join(name.as_ref());
                if let Some(parent) = out_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let file = File::create(&out_path).unwrap();
                Ok(Box::new(BufWriter::new(file)) as Box<dyn Write>)
            }).map_err(|e| format!("{:?}", e))?;
            return Ok(());
        }

        Err("No RAR parser matched".into())
    }
}

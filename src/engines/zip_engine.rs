use crate::engines::traits::{Engine, ArchiveInfo};
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;

pub struct ZipEngine;

impl Engine for ZipEngine {
    fn probe(&self, path: &Path) -> Result<ArchiveInfo, String> {
        // Basic probe: check signature
        let data = fs::read(path).map_err(|e| e.to_string())?;
        if !data.starts_with(b"PK\x03\x04") && !data.starts_with(b"PK\x05\x06") && !data.starts_with(b"PK\x07\x08") {
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

    fn extract(&self, path: &Path, dest: &Path) -> Result<(), String> {
        fs::create_dir_all(dest).map_err(|e| e.to_string())?;
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = entry.name().map_err(|e| e.to_string())?;
            let name_str = name.to_string();
            let out_path = dest.join(&name_str);
            if name_str.ends_with('/') {
                fs::create_dir_all(&out_path).map_err(|e| e.to_string())?;
                continue;
            }
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
            let mut outfile = File::create(&out_path).map_err(|e| e.to_string())?;
            copy(&mut entry, &mut outfile).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

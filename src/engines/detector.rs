use std::path::Path;
use infer;

pub enum ArchiveFamily {
    Zip,
    Rar,
    SevenZip,
    Tar,
    Gzip,
    Bzip2,
    Xz,
    Unknown,
}

pub fn detect_family(path: &Path) -> ArchiveFamily {
    if let Ok(data) = std::fs::read(path).and_then(|mut d| {
        // read first 2KB for detection
        if d.len() > 2048 { d.truncate(2048); }
        Ok(d)
    }) {
        // Try infer
        if let Some(kind) = infer::get(&data) {
            match kind.mime_type() {
                "application/zip" => return ArchiveFamily::Zip,
                "application/gzip" => return ArchiveFamily::Gzip,
                "application/x-bzip2" => return ArchiveFamily::Bzip2,
                "application/x-xz" => return ArchiveFamily::Xz,
                _ => {}
            }
        }
        // RAR signature
        if data.starts_with(b"Rar!") {
            return ArchiveFamily::Rar;
        }
        // 7z signature
        if data.starts_with(b"7z\xbc\xaf\x27\x1c") {
            return ArchiveFamily::SevenZip;
        }
        // tar magic
        if data.len() >= 262 && &data[257..262] == b"ustar" {
            return ArchiveFamily::Tar;
        }
    }
    ArchiveFamily::Unknown
}

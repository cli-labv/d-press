use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy)]
pub enum ExternalTool {
    SevenZip(&'static str),
    Unzip,
}

impl ExternalTool {
    pub fn name(self) -> &'static str {
        match self {
            ExternalTool::SevenZip(cmd) => cmd,
            ExternalTool::Unzip => "unzip",
        }
    }
}

pub fn for_zip() -> Option<ExternalTool> {
    for cmd in ["7zz", "7z", "7za"] {
        if available(cmd) {
            return Some(ExternalTool::SevenZip(cmd));
        }
    }
    if available("unzip") {
        return Some(ExternalTool::Unzip);
    }
    None
}

pub fn for_rar() -> Option<ExternalTool> {
    for cmd in ["7zz", "7z", "7za"] {
        if available(cmd) {
            return Some(ExternalTool::SevenZip(cmd));
        }
    }
    None
}

pub fn extract(tool: ExternalTool, archive: &Path, dest: &Path) -> Result<(), String> {
    let output = match tool {
        ExternalTool::SevenZip(cmd) => Command::new(cmd)
            .arg("x")
            .arg("-y")
            .arg("-aoa")
            .arg(format!("-o{}", dest.display()))
            .arg(archive)
            .output(),
        ExternalTool::Unzip => Command::new("unzip")
            .arg("-o")
            .arg("-qq")
            .arg(archive)
            .arg("-d")
            .arg(dest)
            .output(),
    }
    .map_err(|e| format!("no se pudo ejecutar {}: {e}", tool.name()))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let detail = if stderr.trim().is_empty() {
        stdout
    } else {
        stderr
    };
    Err(format!(
        "{} terminó con {}: {}",
        tool.name(),
        output.status,
        detail.trim()
    ))
}

fn available(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--help")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success() || status.code().is_some())
        .unwrap_or(false)
}

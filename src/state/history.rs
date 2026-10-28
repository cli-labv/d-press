use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn append_history(log_path: &Path, entry: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

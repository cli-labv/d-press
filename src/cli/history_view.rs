use std::path::Path;

pub fn show_history(log_path: &Path) {
    if !log_path.exists() {
        println!("Sin historial.");
        return;
    }
    println!("Historial de corridas:");
    if let Ok(content) = std::fs::read_to_string(log_path) {
        for line in content.lines() {
            println!("{}", line);
        }
    }
}

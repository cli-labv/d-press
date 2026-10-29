use dialoguer::{theme::ColorfulTheme, Select};
use console::{style, Color, Style};
use std::path::Path;

fn menu_theme() -> ColorfulTheme {
    let mut theme = ColorfulTheme::default();
    theme.prompt_style = Style::new().for_stderr().bold().fg(Color::Cyan);
    theme.prompt_prefix = style("?".to_string()).for_stderr().bold().fg(Color::Yellow);
    theme.prompt_suffix = style("›".to_string()).for_stderr().bold().fg(Color::White);
    theme.values_style = Style::new().for_stderr().bold().fg(Color::White);
    theme.active_item_style = Style::new().for_stderr().bold().fg(Color::Cyan);
    theme.inactive_item_style = Style::new().for_stderr().bold().fg(Color::White);
    theme.active_item_prefix = style("❯".to_string()).for_stderr().bold().fg(Color::Green);
    theme.inactive_item_prefix = style(" ".to_string()).for_stderr().bold().fg(Color::White);
    theme.defaults_style = Style::new().for_stderr().bold().fg(Color::Yellow);
    theme.hint_style = Style::new().for_stderr().bold().fg(Color::White);
    theme.error_style = Style::new().for_stderr().bold().fg(Color::Red);
    theme
}

fn has_pending() -> bool {
    let state_dir = Path::new("registros/state");
    if !state_dir.exists() { return false; }
    if let Ok(entries) = std::fs::read_dir(state_dir) {
        for e in entries.flatten() {
            if let Ok(ft) = e.file_type() {
                if ft.is_file() && e.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    return true;
                }
            }
        }
    }
    false
}

pub fn main_menu_adaptive() -> &'static str {
    let mut items: Vec<&'static str> = vec!["Procesar ./input", "Carpeta externa", "Historial"];
    if has_pending() {
        items.push("Reanudar");
    }
    items.push("Salir");

    let theme = menu_theme();
    let selection = Select::with_theme(&theme)
        .with_prompt("d-press")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    items[selection]
}

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use console::{style, Color, Style};
use std::path::PathBuf;

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

pub fn main_menu() -> &'static str {
    let items = &[
        "Procesar ./input",
        "Carpeta externa",
        "Historial",
        "Reanudar",
        "Salir",
    ];
    let theme = menu_theme();
    let selection = Select::with_theme(&theme)
        .with_prompt("d-press")
        .items(items)
        .default(0)
        .interact()
        .unwrap();
    items[selection]
}

pub fn ask_input_output() -> (PathBuf, PathBuf) {
    let theme = menu_theme();
    let input: String = Input::with_theme(&theme)
        .with_prompt("Entrada")
        .default("./input".into())
        .interact_text()
        .unwrap();
    let output: String = Input::with_theme(&theme)
        .with_prompt("Salida")
        .default("./output".into())
        .interact_text()
        .unwrap();
    (PathBuf::from(input), PathBuf::from(output))
}

pub fn ask_mode() -> bool {
    let items = &["B · Preservar estructura", "A · Una carpeta"];
    let theme = menu_theme();
    let sel = Select::with_theme(&theme)
        .with_prompt("Modo")
        .items(items)
        .default(0)
        .interact()
        .unwrap();
    sel == 1
}

pub fn confirm_extract() -> bool {
    Confirm::with_theme(&menu_theme())
        .with_prompt("Extraer")
        .default(true)
        .interact()
        .unwrap_or(false)
}

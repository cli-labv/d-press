use dialoguer::{theme::ColorfulTheme, Input, Select};
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
        "Procesar carpeta ./input",
        "Seleccionar carpeta externa",
        "Ver historial",
        "Reanudar corrida pendiente",
        "Salir",
    ];
    let theme = menu_theme();
    let selection = Select::with_theme(&theme)
        .with_prompt("d-press · menú principal")
        .items(items)
        .default(0)
        .interact()
        .unwrap();
    items[selection]
}

pub fn ask_input_output() -> (PathBuf, PathBuf) {
    let theme = menu_theme();
    let input: String = Input::with_theme(&theme)
        .with_prompt("Carpeta de entrada")
        .default("./input".into())
        .interact_text()
        .unwrap();
    let output: String = Input::with_theme(&theme)
        .with_prompt("Carpeta de salida")
        .default("./output".into())
        .interact_text()
        .unwrap();
    (PathBuf::from(input), PathBuf::from(output))
}

pub fn ask_mode() -> bool {
    let items = &["Modo A - Una carpeta de salida", "Modo B - Preservar estructura"];
    let theme = menu_theme();
    let sel = Select::with_theme(&theme)
        .with_prompt("Modo de salida")
        .items(items)
        .default(0)
        .interact()
        .unwrap();
    sel == 0
}

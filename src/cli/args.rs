use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "d-press", about = "Descompresión masiva")]
pub struct Args {
    #[arg(short, long, help = "Carpeta de entrada")]
    pub input: Option<PathBuf>,

    #[arg(short, long, help = "Carpeta de salida")]
    pub output: Option<PathBuf>,

    #[arg(short, long, default_value = "flat", help = "Modo de salida: flat o preserve")]
    pub mode: String,

    #[arg(long, default_value_t = false, help = "No preguntar confirmación")]
    pub yes: bool,

    #[arg(long = "no-banner", default_value_t = false, help = "No mostrar banner")]
    pub no_banner: bool,

    #[arg(long = "simple-banner", default_value_t = false, help = "Banner simple")]
    pub simple_banner: bool,

    #[arg(long = "compact-banner", default_value_t = false, help = "Banner compacto")]
    pub compact_banner: bool,

    #[arg(long = "ascii-only", default_value_t = false, help = "Banner ASCII")]
    pub ascii_only: bool,

    #[arg(long = "no-color", default_value_t = false, help = "No usar colores")]
    pub no_color: bool,

    #[arg(long = "no-animation", default_value_t = false, help = "No animar banner")]
    pub no_animation: bool,

    #[arg(long = "banner-theme", value_name = "THEME", help = "Tema de banner")]
    pub banner_theme: Option<String>,

    #[arg(long = "fast-banner", default_value_t = false, help = "Banner rapido")]
    pub fast_banner: bool,

    #[arg(long = "cinematic-banner", default_value_t = false, help = "Banner cinematografico")]
    pub cinematic_banner: bool,
}

impl Args {
    pub fn has_inputs(&self) -> bool {
        self.input.is_some() && self.output.is_some()
    }
}

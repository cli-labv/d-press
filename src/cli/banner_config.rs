use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationCfg {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_true")]
    pub typing: bool,
    #[serde(default = "default_six")]
    pub char_delay_ms: u64,
    #[serde(default = "default_sixteen")]
    pub line_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorsCfg {
    #[serde(default)]
    pub logo: Vec<String>,
    #[serde(default = "default_cyan")]
    pub tagline: String,
    #[serde(default = "default_magenta")]
    pub border: String,
    #[serde(default = "default_dim")]
    pub footer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BannerFileConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_neon")]
    pub theme: String,
    #[serde(default = "default_rounded")]
    pub style: String,
    #[serde(default = "default_true")]
    pub show_tagline: bool,
    #[serde(default = "default_true")]
    pub show_version: bool,
    #[serde(default = "default_true")]
    pub show_separator: bool,
    #[serde(default)]
    pub animation: AnimationCfg,
    #[serde(default)]
    pub colors: ColorsCfg,
    #[serde(default)]
    pub tagline: Vec<String>,
    #[serde(default = "default_footer")]
    pub footer: String,
}

fn default_true() -> bool { true }
fn default_six() -> u64 { 6 }
fn default_sixteen() -> u64 { 16 }
fn default_neon() -> String { "neon".to_string() }
fn default_rounded() -> String { "rounded".to_string() }
fn default_cyan() -> String { "cyan_bright".to_string() }
fn default_magenta() -> String { "magenta_bright".to_string() }
fn default_dim() -> String { "dim".to_string() }
fn default_footer() -> String { "v0.1.0 │ Portable CLI".to_string() }

impl Default for AnimationCfg {
    fn default() -> Self {
        Self { enabled: true, typing: true, char_delay_ms: 6, line_delay_ms: 16 }
    }
}

impl Default for ColorsCfg {
    fn default() -> Self {
        Self {
            logo: vec!["cyan_bright".to_string(), "magenta_bright".to_string(), "cyan".to_string(), "blue_bright".to_string(), "white".to_string()],
            tagline: "cyan_bright".to_string(),
            border: "magenta_bright".to_string(),
            footer: "dim".to_string(),
        }
    }
}

impl Default for BannerFileConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            theme: "neon".to_string(),
            style: "rounded".to_string(),
            show_tagline: true,
            show_version: true,
            show_separator: true,
            animation: AnimationCfg::default(),
            colors: ColorsCfg::default(),
            tagline: vec![
                "Descompresión masiva · Rápido · Paralelo · Seguro".to_string(),
                "ZIP · RAR4/RAR5 · 7Z · TAR · GZ · BZ2 · XZ".to_string(),
            ],
            footer: "v0.1.0 │ Portable CLI".to_string(),
        }
    }
}

pub fn load_banner_config() -> BannerFileConfig {
    let paths = ["banner.config.json", ".skills/banner.config.json", "config/banner.config.json"];
    for p in &paths {
        if Path::new(p).exists() {
            if let Ok(s) = fs::read_to_string(p) {
                if let Ok(cfg) = serde_json::from_str::<BannerFileConfig>(&s) {
                    return cfg;
                }
            }
        }
    }
    BannerFileConfig::default()
}

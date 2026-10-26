use console::{colors_enabled, measure_text_width, Term};
use std::env;
use std::io::{self, Write};
use std::time::Duration;

const MAX_CHAR_DELAY_MS: u64 = 100;
const MAX_LINE_DELAY_MS: u64 = 2_000;
const MAX_BANNER_WIDTH: usize = 160;
const MIN_TERMINAL_WIDTH: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerColor {
    Default,
    Black,
    Red,
    RedBright,
    Yellow,
    YellowBright,
    Green,
    GreenBright,
    Cyan,
    CyanBright,
    Blue,
    BlueBright,
    Magenta,
    MagentaBright,
    White,
    WhiteBright,
    Gray,
    Dim,
}

impl BannerColor {
    fn ansi_code(&self) -> Option<&'static str> {
        Some(match self {
            Self::Default => return None,
            Self::Black => "30",
            Self::Red => "31",
            Self::RedBright => "91",
            Self::Yellow => "33",
            Self::YellowBright => "93",
            Self::Green => "32",
            Self::GreenBright => "92",
            Self::Cyan => "36",
            Self::CyanBright => "96",
            Self::Blue => "34",
            Self::BlueBright => "94",
            Self::Magenta => "35",
            Self::MagentaBright => "95",
            Self::White => "37",
            Self::WhiteBright => "97",
            Self::Gray => "90",
            Self::Dim => "2",
        })
    }

    fn apply(&self, text: &str, enabled: bool) -> String {
        if !enabled {
            return text.to_string();
        }
        let Some(code) = self.ansi_code() else {
            return text.to_string();
        };
        format!("\x1b[{}m{}\x1b[0m", code, text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BannerLine {
    pub content: String,
    pub color: BannerColor,
    pub delay_ms: u64,
    pub typing: bool,
}

impl BannerLine {
    pub fn new(content: impl Into<String>, color: BannerColor, delay_ms: u64, typing: bool) -> Self {
        Self {
            content: content.into(),
            color,
            delay_ms,
            typing,
        }
    }

    pub fn static_line(content: impl Into<String>, color: BannerColor, delay_ms: u64) -> Self {
        Self::new(content, color, delay_ms, false)
    }

    pub fn typing_line(content: impl Into<String>, color: BannerColor, delay_ms: u64) -> Self {
        Self::new(content, color, delay_ms, true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BannerDefinition {
    pub name: &'static str,
    pub logo: Vec<BannerLine>,
    pub tagline: Vec<BannerLine>,
    pub footer: Vec<BannerLine>,
    pub separator: Option<BannerLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BannerAnimationConfig {
    pub enabled: bool,
    pub typing_enabled: bool,
    pub char_delay_ms: u64,
    pub line_delay_ms: u64,
    pub dramatic_pause_ms: u64,
    pub tagline_delay_ms: u64,
    pub footer_delay_ms: u64,
}

impl Default for BannerAnimationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            typing_enabled: true,
            char_delay_ms: 8,
            line_delay_ms: 20,
            dramatic_pause_ms: 100,
            tagline_delay_ms: 40,
            footer_delay_ms: 0,
        }
    }
}

impl BannerAnimationConfig {
    pub fn from_env() -> Self {
        let enabled = env_bool("BANNER_ENABLED", true);
        let typing_enabled = env_bool("BANNER_TYPING", true);
        let char_delay_ms = env_u64("BANNER_CHAR_DELAY_MS", 8).min(MAX_CHAR_DELAY_MS);
        let line_delay_ms = env_u64("BANNER_LINE_DELAY_MS", 20).min(MAX_LINE_DELAY_MS);
        let dramatic_pause_ms = env_u64("BANNER_DRAMATIC_PAUSE_MS", 100).min(MAX_LINE_DELAY_MS);
        let tagline_delay_ms = env_u64("BANNER_TAGLINE_DELAY_MS", 40).min(MAX_LINE_DELAY_MS);
        let footer_delay_ms = env_u64("BANNER_FOOTER_DELAY_MS", 0).min(MAX_LINE_DELAY_MS);
        Self {
            enabled,
            typing_enabled,
            char_delay_ms,
            line_delay_ms,
            dramatic_pause_ms,
            tagline_delay_ms,
            footer_delay_ms,
        }
        .normalized()
    }

    pub fn normalized(&self) -> Self {
        Self {
            enabled: self.enabled,
            typing_enabled: self.typing_enabled,
            char_delay_ms: self.char_delay_ms.min(MAX_CHAR_DELAY_MS),
            line_delay_ms: self.line_delay_ms.min(MAX_LINE_DELAY_MS),
            dramatic_pause_ms: self.dramatic_pause_ms.min(MAX_LINE_DELAY_MS),
            tagline_delay_ms: self.tagline_delay_ms.min(MAX_LINE_DELAY_MS),
            footer_delay_ms: self.footer_delay_ms.min(MAX_LINE_DELAY_MS),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxStyle {
    Rounded,
    Double,
    Square,
    Minimal,
    Unicode,
    Ascii,
}

#[derive(Debug, Clone)]
pub struct BannerTheme {
    pub name: &'static str,
    pub logo_colors: Vec<BannerColor>,
    pub tagline_color: BannerColor,
    pub border_color: BannerColor,
    pub footer_color: BannerColor,
    pub separator_color: BannerColor,
    pub box_style: BoxStyle,
}

pub fn available_themes() -> Vec<BannerTheme> {
    vec![
        BannerTheme {
            name: "neon",
            logo_colors: vec![
                BannerColor::CyanBright,
                BannerColor::MagentaBright,
                BannerColor::Cyan,
                BannerColor::BlueBright,
                BannerColor::White,
            ],
            tagline_color: BannerColor::CyanBright,
            border_color: BannerColor::MagentaBright,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Rounded,
        },
        BannerTheme {
            name: "ocean",
            logo_colors: vec![
                BannerColor::BlueBright,
                BannerColor::Cyan,
                BannerColor::Blue,
                BannerColor::CyanBright,
                BannerColor::White,
            ],
            tagline_color: BannerColor::Cyan,
            border_color: BannerColor::Blue,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Rounded,
        },
        BannerTheme {
            name: "terminal",
            logo_colors: vec![
                BannerColor::GreenBright,
                BannerColor::CyanBright,
                BannerColor::Green,
                BannerColor::Cyan,
                BannerColor::White,
            ],
            tagline_color: BannerColor::GreenBright,
            border_color: BannerColor::CyanBright,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Rounded,
        },
        BannerTheme {
            name: "fire",
            logo_colors: vec![
                BannerColor::RedBright,
                BannerColor::YellowBright,
                BannerColor::Red,
                BannerColor::Yellow,
                BannerColor::White,
            ],
            tagline_color: BannerColor::RedBright,
            border_color: BannerColor::YellowBright,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Rounded,
        },
        BannerTheme {
            name: "monochrome",
            logo_colors: vec![
                BannerColor::White,
                BannerColor::WhiteBright,
                BannerColor::Gray,
                BannerColor::White,
                BannerColor::WhiteBright,
            ],
            tagline_color: BannerColor::White,
            border_color: BannerColor::WhiteBright,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Square,
        },
        BannerTheme {
            name: "extraction",
            logo_colors: vec![
                BannerColor::CyanBright,
                BannerColor::GreenBright,
                BannerColor::Cyan,
                BannerColor::Green,
                BannerColor::White,
            ],
            tagline_color: BannerColor::CyanBright,
            border_color: BannerColor::GreenBright,
            footer_color: BannerColor::Dim,
            separator_color: BannerColor::Dim,
            box_style: BoxStyle::Rounded,
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerVariant {
    Full,
    Compact,
    Minimal,
    Success,
    Error,
    Goodbye,
    Processing,
    Completed,
}

#[derive(Debug, Clone)]
pub struct BannerConfig {
    pub definition: BannerDefinition,
    pub animation: BannerAnimationConfig,
    pub enabled: bool,
    pub simple: bool,
    pub compact: bool,
    pub ascii_only: bool,
    pub no_color: bool,
    pub no_animation: bool,
    pub theme: BannerTheme,
    pub variant: BannerVariant,
}

impl Default for BannerConfig {
    fn default() -> Self {
        Self {
            definition: dpress_banner(),
            animation: BannerAnimationConfig::default(),
            enabled: true,
            simple: false,
            compact: false,
            ascii_only: false,
            no_color: false,
            no_animation: false,
            theme: BannerTheme {
                name: "neon",
                logo_colors: vec![
                    BannerColor::CyanBright,
                    BannerColor::MagentaBright,
                    BannerColor::Cyan,
                    BannerColor::BlueBright,
                    BannerColor::White,
                ],
                tagline_color: BannerColor::CyanBright,
                border_color: BannerColor::MagentaBright,
                footer_color: BannerColor::Dim,
                separator_color: BannerColor::Dim,
                box_style: BoxStyle::Rounded,
            },
            variant: BannerVariant::Full,
        }
    }
}

impl BannerConfig {
    pub fn from_args(args: &crate::cli::args::Args) -> Self {
        let animation = if args.no_animation {
            BannerAnimationConfig {
                enabled: false,
                ..BannerAnimationConfig::default()
            }
        } else if args.fast_banner {
            BannerAnimationConfig {
                char_delay_ms: 2,
                line_delay_ms: 5,
                dramatic_pause_ms: 30,
                tagline_delay_ms: 10,
                footer_delay_ms: 0,
                ..BannerAnimationConfig::default()
            }
        } else if args.cinematic_banner {
            BannerAnimationConfig {
                char_delay_ms: 12,
                line_delay_ms: 30,
                dramatic_pause_ms: 200,
                tagline_delay_ms: 60,
                footer_delay_ms: 20,
                ..BannerAnimationConfig::default()
            }
        } else {
            BannerAnimationConfig::from_env()
        };
        let animation = animation.normalized();
        let theme_name = args.banner_theme.clone().unwrap_or_else(|| "neon".to_string());
        let themes = available_themes();
        let theme = themes
            .iter()
            .find(|t| t.name == theme_name.as_str())
            .cloned()
            .unwrap_or_else(|| themes.first().expect("themes").clone());

        Self {
            definition: dpress_banner(),
            animation,
            enabled: !args.no_banner,
            simple: args.simple_banner,
            compact: args.compact_banner,
            ascii_only: args.ascii_only,
            no_color: args.no_color,
            no_animation: args.no_animation,
            theme,
            variant: BannerVariant::Full,
        }
    }
}

pub fn should_render_animated_banner(config: &BannerConfig) -> bool {
    if !config.enabled || config.no_color || config.no_animation || !config.animation.enabled {
        return false;
    }
    if !Term::stdout().is_term() {
        return false;
    }
    if !colors_enabled() {
        return false;
    }
    if env::var("NO_COLOR").is_ok() {
        return false;
    }
    if env::var("CI").is_ok() {
        return false;
    }
    let (columns, _) = Term::stdout().size();
    if (columns as usize) < MIN_TERMINAL_WIDTH {
        return false;
    }
    if visible_width(config.definition.name) > MAX_BANNER_WIDTH {
        return false;
    }
    true
}

pub fn show_animated_banner(config: &BannerConfig) {
    if !should_render_animated_banner(config) {
        return;
    }
    let enabled_color = !config.no_color && colors_enabled() && Term::stdout().is_term();
    let _ = render_top_decoration(&config.animation, enabled_color);
    let icon = BannerLine::new("❯", config.theme.tagline_color, config.animation.dramatic_pause_ms, false);
    let _ = render_line(&icon, &config.animation, enabled_color);
    let _ = render_logo(config, enabled_color);
    pause(config.animation.dramatic_pause_ms, config.animation.enabled);
    let _ = render_tagline_box(config, enabled_color);
    let _ = render_version(config, enabled_color);
    let _ = render_separator(config, enabled_color);
}

pub fn show_simple_banner(config: &BannerConfig) {
    if !config.enabled || config.no_color {
        return;
    }
    let enabled_color = !config.no_color && colors_enabled() && Term::stdout().is_term();
    render_static(config, enabled_color);
}

pub fn show_compact_banner(config: &BannerConfig) {
    let compact = BannerConfig {
        definition: BannerDefinition {
            tagline: vec![BannerLine::new(
                "Descompresión masiva y segura",
                config.theme.tagline_color,
                0,
                false,
            )],
            footer: vec![BannerLine::new("v0.1.0", config.theme.footer_color, 0, false)],
            ..config.definition.clone()
        },
        compact: true,
        ..config.clone()
    };
    show_simple_banner(&compact);
}

pub fn show_processing_banner(config: &BannerConfig) {
    let processing = BannerConfig {
        definition: BannerDefinition {
            tagline: vec![BannerLine::new(
                "Procesando archivos comprimidos...",
                config.theme.tagline_color,
                0,
                false,
            )],
            footer: vec![BannerLine::new("d-press", config.theme.footer_color, 0, false)],
            ..config.definition.clone()
        },
        variant: BannerVariant::Processing,
        ..config.clone()
    };
    show_simple_banner(&processing);
}

pub fn show_completed_banner(config: &BannerConfig) {
    let completed = BannerConfig {
        definition: BannerDefinition {
            tagline: vec![BannerLine::new(
                "✓ Archivos liberados correctamente",
                config.theme.tagline_color,
                0,
                false,
            )],
            footer: vec![BannerLine::new("d-press", config.theme.footer_color, 0, false)],
            ..config.definition.clone()
        },
        variant: BannerVariant::Completed,
        ..config.clone()
    };
    show_simple_banner(&completed);
}

pub fn show_goodbye_banner(config: &BannerConfig) {
    let goodbye = BannerConfig {
        definition: BannerDefinition {
            tagline: vec![
                BannerLine::new(
                    "◇ Gracias por usar d-press ◇",
                    config.theme.tagline_color,
                    0,
                    false,
                ),
                BannerLine::new(
                    "  Archivos liberados correctamente",
                    config.theme.footer_color,
                    0,
                    false,
                ),
            ],
            footer: vec![BannerLine::new("d-press", config.theme.footer_color, 0, false)],
            ..config.definition.clone()
        },
        variant: BannerVariant::Goodbye,
        ..config.clone()
    };
    show_simple_banner(&goodbye);
}

pub fn print_banner(config: &BannerConfig) {
    if config.simple {
        show_simple_banner(config);
    } else if config.compact {
        show_compact_banner(config);
    } else {
        show_animated_banner(config);
    }
}

fn render_top_decoration(
    animation: &BannerAnimationConfig,
    enabled_color: bool,
) -> io::Result<()> {
    let line = BannerLine::new("─".repeat(40), BannerColor::Dim, animation.line_delay_ms, false);
    render_line(&line, animation, enabled_color)
}

fn render_logo(config: &BannerConfig, enabled_color: bool) -> io::Result<()> {
    let animation = &config.animation;
    for line in &config.definition.logo {
        let delay_ms = if line.typing {
            animation.char_delay_ms
        } else {
            animation.line_delay_ms
        };
        let _ = render_line_with_delay(line, delay_ms, animation, enabled_color);
    }
    Ok(())
}

fn render_tagline_box(config: &BannerConfig, enabled_color: bool) -> io::Result<()> {
    let animation = &config.animation;
    let style = if config.ascii_only {
        BoxStyle::Ascii
    } else {
        config.theme.box_style
    };
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = match style {
        BoxStyle::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
        BoxStyle::Double => ('╔', '╗', '╚', '╝', '═', '║'),
        BoxStyle::Square => ('+', '+', '+', '+', '-', '|'),
        BoxStyle::Minimal => ('┌', '┐', '└', '┘', '─', '│'),
        BoxStyle::Unicode => ('╭', '╮', '╰', '╯', '─', '│'),
        BoxStyle::Ascii => ('+', '+', '+', '+', '-', '|'),
    };
    let _ = (top_left, top_right, bottom_left, bottom_right, horizontal, vertical);
    let box_lines = build_box(&config.definition.tagline, 0, horizontal, vertical);
    for (index, content) in box_lines.iter().enumerate() {
        let color = if index == 0 || index == box_lines.len() - 1 {
            config.theme.border_color
        } else {
            config.theme.tagline_color
        };
        let line = BannerLine::new(content.clone(), color, animation.tagline_delay_ms, false);
        let _ = render_line_with_delay(&line, animation.tagline_delay_ms, animation, enabled_color);
    }
    Ok(())
}

fn render_version(config: &BannerConfig, enabled_color: bool) -> io::Result<()> {
    let animation = &config.animation;
    for line in &config.definition.footer {
        let _ = render_line_with_delay(line, animation.footer_delay_ms, animation, enabled_color);
    }
    Ok(())
}

fn render_separator(config: &BannerConfig, enabled_color: bool) -> io::Result<()> {
    let animation = &config.animation;
    if let Some(separator) = &config.definition.separator {
        let _ = render_line_with_delay(separator, animation.footer_delay_ms, animation, enabled_color);
    }
    Ok(())
}

fn render_static(config: &BannerConfig, enabled_color: bool) {
    for line in &config.definition.logo {
        println!("{}", line.color.apply(&line.content, enabled_color));
    }
    for line in &config.definition.tagline {
        println!("{}", line.color.apply(&line.content, enabled_color));
    }
    for line in &config.definition.footer {
        println!("{}", line.color.apply(&line.content, enabled_color));
    }
    if let Some(separator) = &config.definition.separator {
        println!("{}", separator.color.apply(&separator.content, enabled_color));
    }
}

fn render_line(
    line: &BannerLine,
    animation: &BannerAnimationConfig,
    enabled_color: bool,
) -> io::Result<()> {
    let delay_ms = if line.typing {
        animation.char_delay_ms
    } else {
        animation.line_delay_ms
    };
    render_line_with_delay(line, delay_ms, animation, enabled_color)
}

fn render_line_with_delay(
    line: &BannerLine,
    delay_ms: u64,
    animation: &BannerAnimationConfig,
    enabled_color: bool,
) -> io::Result<()> {
    let text = line.content.as_str();
    if !enabled_color {
        println!("{}", text);
        io::stdout().flush()?;
        return Ok(());
    }
    if line.typing && animation.typing_enabled && animation.enabled {
        animate_typing(text, line.color, animation.char_delay_ms, animation.enabled, animation.typing_enabled)
    } else {
        animate_line(text, line.color, delay_ms, animation.enabled)
    }
}

fn animate_typing(
    text: &str,
    color: BannerColor,
    char_delay_ms: u64,
    enabled: bool,
    skip_whitespace_delay: bool,
) -> io::Result<()> {
    if !enabled {
        println!("{}", color.apply(text, true));
        io::stdout().flush()?;
        return Ok(());
    }
    let colored = color.apply(text, true);
    for character in colored.chars() {
        print!("{character}");
        io::stdout().flush()?;
        let wait = !character.is_whitespace() || !skip_whitespace_delay;
        if wait {
            std::thread::sleep(Duration::from_millis(char_delay_ms));
        }
    }
    println!();
    Ok(())
}

fn animate_line(text: &str, color: BannerColor, delay_ms: u64, enabled: bool) -> io::Result<()> {
    if !enabled {
        println!("{}", color.apply(text, true));
        io::stdout().flush()?;
        return Ok(());
    }
    println!("{}", color.apply(text, true));
    io::stdout().flush()?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    Ok(())
}

fn pause(ms: u64, enabled: bool) {
    if enabled && ms > 0 {
        std::thread::sleep(Duration::from_millis(ms));
    }
}

fn visible_width(text: &str) -> usize {
    measure_text_width(text)
}

fn pad_center_to_width(text: &str, width: usize) -> String {
    let text_width = visible_width(text);
    if text_width >= width {
        return text.to_string();
    }
    let remaining = width - text_width;
    let left = remaining / 2;
    let right = remaining - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

fn pad_to_width(text: &str, width: usize) -> String {
    format!("{}{}", text, " ".repeat(width.saturating_sub(visible_width(text))))
}

fn build_box(lines: &[BannerLine], indent: usize, horizontal: char, vertical: char) -> Vec<String> {
    let _ = indent;
    let max = lines
        .iter()
        .map(|line| visible_width(&line.content))
        .max()
        .unwrap_or(0);
    let inner = max + 2;
    let top = format!("{}{}{}", vertical, horizontal.to_string().repeat(inner), vertical);
    let bottom = top.clone();
    let mut result = vec![top];
    for line in lines {
        let content = pad_center_to_width(&line.content, max);
        result.push(format!("{} {} {}", vertical, content, vertical));
    }
    result.push(bottom);
    result
}

fn logo_color(index: usize) -> BannerColor {
    match index % 5 {
        0 => BannerColor::CyanBright,
        1 => BannerColor::Cyan,
        2 => BannerColor::Blue,
        3 => BannerColor::BlueBright,
        4 => BannerColor::White,
        _ => BannerColor::CyanBright,
    }
}

fn env_bool(name: &str, default: bool) -> bool {
    match env::var(name) {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            normalized == "1"
                || normalized == "true"
                || normalized == "yes"
                || normalized == "on"
        }
        Err(_) => default,
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

pub fn dpress_banner() -> BannerDefinition {
    let font = figlet_rs::FIGlet::standard().ok();
    let art = match font {
        Some(f) => f
            .convert("d-press")
            .map(|figure| figure.to_string())
            .unwrap_or_else(|| "d-press".to_string()),
        None => "d-press".to_string(),
    };
    let art_lines: Vec<String> = art.lines().map(|line| line.trim_end().to_string()).collect();
    let logo = if art_lines.is_empty() {
        vec![BannerLine::typing_line("d-press", BannerColor::CyanBright, 8)]
    } else {
        art_lines
            .into_iter()
            .enumerate()
            .map(|(index, content)| BannerLine::typing_line(content, logo_color(index), 8))
            .collect()
    };

    BannerDefinition {
        name: "d-press",
        logo,
        tagline: vec![
            BannerLine::static_line(
                "Descompresión masiva y segura",
                BannerColor::CyanBright,
                0,
            ),
            BannerLine::static_line(
                "📦 Portable · ⚡ Paralelo · 🛡 Seguro",
                BannerColor::White,
                0,
            ),
        ],
        footer: vec![BannerLine::static_line("v0.1.0", BannerColor::Dim, 0)],
        separator: Some(BannerLine::static_line(
            "─".repeat(40),
            BannerColor::Dim,
            0,
        )),
    }
}

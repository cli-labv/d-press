use crate::cli::ui::{self, Tone};
use crate::core::planner::Plan;
use crate::core::portable::{NameFix, fix_component};
use crate::engines::detector::{ArchiveFamily, detect_family};
use jwalk::WalkDir;
use std::path::Path;

pub fn print_diagnostics(plan: &Plan) {
    let mut total = 0usize;
    let mut compatible = 0usize;
    let mut incompatible = 0usize;
    let mut special: Vec<(String, NameFix)> = Vec::new();
    for entry in WalkDir::new(&plan.input).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        total += 1;
        match detect_family(&p) {
            ArchiveFamily::Unknown => incompatible += 1,
            _ => {
                compatible += 1;
                if let Some(fix) = folder_fix(&p) {
                    let rel = p
                        .strip_prefix(&plan.input)
                        .map(|r| r.display().to_string())
                        .unwrap_or_else(|_| p.display().to_string());
                    special.push((rel, fix));
                }
            }
        }
    }

    ui::print_rows(
        "Diagnóstico",
        &[
            ("Archivos", total.to_string(), Tone::Normal),
            ("Compatibles", compatible.to_string(), Tone::Good),
            (
                "Especiales",
                special.len().to_string(),
                if special.is_empty() {
                    Tone::Normal
                } else {
                    Tone::Warn
                },
            ),
            (
                "No compatibles",
                incompatible.to_string(),
                if incompatible > 0 {
                    Tone::Bad
                } else {
                    Tone::Normal
                },
            ),
        ],
    );

    if !special.is_empty() {
        if special.len() <= 2 {
            for (path, fix) in &special {
                let name = Path::new(path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.clone());
                eprintln!("  ⚠ {} · {} · solo este archivo", name, fix.reason());
            }
        } else {
            eprintln!(
                "  ⚠ {} archivos · ajuste especial solo en ellos",
                special.len()
            );
        }
        eprintln!();
    }
}

fn folder_fix(path: &Path) -> Option<NameFix> {
    let stem = path.file_stem().and_then(|s| s.to_str())?;
    fix_component(stem)
}

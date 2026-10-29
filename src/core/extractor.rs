use crate::cli::ui::{self, Tone};
use crate::core::planner::{OutputMode, Plan};
use crate::core::portable::fix_component;
use crate::engines::detector::ArchiveFamily;
use crate::engines::detector::detect_family;
use crate::engines::{rar_engine::RarEngine, traits::Engine, zip_engine::ZipEngine};
use crate::state::{
    history,
    run_state::{FileState, RunState, Status},
};
use indicatif::{ProgressBar, ProgressStyle};
use jwalk::WalkDir;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub fn run(plan: &Plan) -> Result<(), String> {
    let started = Instant::now();

    // Prepare state dirs (records alongside output)
    let records_dir = records_dir_for_plan(plan);
    let state_dir = records_dir.join("state");
    let logs_dir = records_dir.join("logs");
    std::fs::create_dir_all(&state_dir).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&logs_dir).map_err(|e| e.to_string())?;

    // Deterministic run id from input/output
    let run_id = format!("{:x}", {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        plan.input.display().to_string().hash(&mut h);
        plan.output.display().to_string().hash(&mut h);
        h.finish()
    });
    let state_path = state_dir.join(format!("{}.json", run_id));
    let history_path = logs_dir.join("historial.jsonl");

    // Load previous state if exists
    let previous: Option<RunState> = RunState::load(&state_path);
    let mut files = if let Some(ref rs) = previous {
        rs.files.clone()
    } else {
        Vec::new()
    };
    let mut already_done = HashSet::new();
    let mut prior_notices: HashMap<String, Vec<String>> = HashMap::new();
    for f in &files {
        if f.status == Status::Extracted {
            already_done.insert(f.path.clone());
            prior_notices.insert(f.path.clone(), f.compatibility.clone());
        }
    }

    // Collect archives to process
    let mut archives = Vec::new();
    for entry in WalkDir::new(&plan.input).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let family = detect_family(&p);
        if matches!(family, ArchiveFamily::Unknown) {
            continue;
        }
        archives.push(p.to_path_buf());
    }

    let pb = ProgressBar::new(archives.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {bar:32.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("━╸─"),
    );

    let mut extracted = 0usize;
    let mut errors = 0usize;
    let mut skipped = 0usize;
    let mut compat: Vec<(String, Vec<String>)> = Vec::new();

    for path in archives {
        let path_str = path.display().to_string();
        if already_done.contains(&path_str) {
            let recorded = prior_notices.get(&path_str);
            let pending_special =
                needs_special(&path) && recorded.map_or(true, |notices| notices.is_empty());
            if !pending_special {
                skipped += 1;
                if let Some(notices) = recorded {
                    if !notices.is_empty() {
                        compat.push((path_str.clone(), notices.clone()));
                    }
                }
                pb.inc(1);
                pb.set_message(format!("omitido {}", short_name(&path)));
                continue;
            }
            pb.suspend(|| {
                eprintln!("↻ {} · reextraer para aplicar compatibilidad", path_str);
            });
        }
        let family = detect_family(&path);
        let engine: Box<dyn Engine> = match family {
            ArchiveFamily::Rar => Box::new(RarEngine),
            ArchiveFamily::Zip => Box::new(ZipEngine),
            _ => {
                if let Some(tool) = crate::engines::external::for_generic() {
                    Box::new(crate::engines::external::ExternalEngine::new(tool))
                } else {
                    // No external tool available, skip
                    continue;
                }
            }
        };
        let (dest, mut notices) = target_dir(plan, &path)?;
        let state_entry = match engine.extract(&path, &dest) {
            Ok(outcome) => {
                notices.extend(outcome.notices);
                if !notices.is_empty() {
                    compat.push((path_str.clone(), notices.clone()));
                }
                extracted += 1;
                pb.inc(1);
                pb.set_message(format!("✓ {}", short_name(&path)));
                FileState {
                    path: path_str.clone(),
                    status: Status::Extracted,
                    compatibility: notices,
                }
            }
            Err(e) => {
                if !notices.is_empty() {
                    compat.push((path_str.clone(), notices.clone()));
                }
                errors += 1;
                pb.suspend(|| eprintln!("✖ {} · {}", short_name(&path), e));
                pb.inc(1);
                pb.set_message(format!("✖ {}", short_name(&path)));
                FileState {
                    path: path_str.clone(),
                    status: Status::Error,
                    compatibility: notices,
                }
            }
        };
        files.retain(|f| f.path != path_str);
        files.push(state_entry);
    }
    pb.finish_and_clear();

    // Save run state
    let run_state = RunState {
        id: run_id.clone(),
        files,
    };
    run_state.save(&state_path).map_err(|e| e.to_string())?;

    // Append history only on first run
    if previous.is_none() {
        let entry = format!(
            "{{\"id\":\"{}\",\"input\":\"{}\",\"output\":\"{}\",\"ts\":{}}}",
            run_id,
            plan.input.display(),
            plan.output.display(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        history::append_history(&history_path, &entry).map_err(|e| e.to_string())?;
    }

    print_summary(
        extracted,
        errors,
        skipped,
        &compat,
        started.elapsed().as_secs_f64(),
    );
    Ok(())
}

fn short_name(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    console::truncate_str(&name, 40, "…").to_string()
}

fn print_summary(
    extracted: usize,
    errors: usize,
    skipped: usize,
    compat: &[(String, Vec<String>)],
    seconds: f64,
) {
    let mut rows: Vec<(&str, String, Tone)> = vec![
        ("Extraídos", extracted.to_string(), Tone::Good),
        (
            "Errores",
            errors.to_string(),
            if errors > 0 { Tone::Bad } else { Tone::Normal },
        ),
    ];
    if skipped > 0 {
        rows.push(("Omitidos", skipped.to_string(), Tone::Normal));
    }
    rows.push((
        "Especiales",
        compat.len().to_string(),
        if compat.is_empty() {
            Tone::Normal
        } else {
            Tone::Warn
        },
    ));
    rows.push(("Tiempo", format!("{seconds:.1} s"), Tone::Normal));
    ui::print_rows("Resultado", &rows);

    if compat.is_empty() {
        return;
    }

    let mut lines = Vec::new();
    for (index, (path, notices)) in compat.iter().take(3).enumerate() {
        if index > 0 {
            lines.push(String::new());
        }
        lines.push(short_name(Path::new(path)));
        for notice in notices.iter().take(2) {
            lines.push(format!("  {notice}"));
        }
        if notices.len() > 2 {
            lines.push(format!("  +{} ajustes", notices.len() - 2));
        }
    }
    if compat.len() > 3 {
        lines.push(format!("+{} archivos más", compat.len() - 3));
    }
    lines.push("Solo estos archivos · no es el modo normal".to_string());
    ui::print_notice("Compatibilidad especial", &lines);
}

fn needs_special(path: &Path) -> bool {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(fix_component)
        .is_some()
}

fn records_dir_for_plan(plan: &Plan) -> PathBuf {
    if let Some(parent) = plan.output.parent() {
        parent.join("registros")
    } else {
        PathBuf::from("registros")
    }
}

fn target_dir(plan: &Plan, archive_path: &Path) -> Result<(PathBuf, Vec<String>), String> {
    let rel = archive_path
        .strip_prefix(&plan.input)
        .map_err(|e| e.to_string())?;
    let stem = archive_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("archive");
    let mut notices = Vec::new();
    let base_name = match fix_component(stem) {
        Some(fix) => {
            notices.push(format!("carpeta «{}» · {}", fix.value, fix.reason()));
            fix.value
        }
        None => stem.to_string(),
    };
    let folder = match plan.mode {
        OutputMode::Flat => plan.output.join(base_name),
        OutputMode::Preserve => {
            let parent = rel.parent().unwrap_or(Path::new(""));
            plan.output.join(parent).join(base_name)
        }
    };
    // Simple collision avoidance
    let mut candidate = folder.clone();
    let mut i = 1;
    while candidate.exists() {
        let name = folder.file_name().unwrap().to_string_lossy();
        candidate = folder.with_file_name(format!("{}_{i}", name));
        i += 1;
    }
    std::fs::create_dir_all(&candidate).map_err(|e| e.to_string())?;
    Ok((candidate, notices))
}

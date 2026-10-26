use crate::core::planner::{Plan, OutputMode};
use crate::engines::{rar_engine::RarEngine, zip_engine::ZipEngine, traits::Engine};
use crate::engines::detector::detect_family;
use crate::engines::detector::ArchiveFamily;
use crate::state::{history, run_state::{RunState, FileState, Status}};
use jwalk::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run(plan: &Plan) -> Result<(), String> {
    // Prepare state dirs
    let state_dir = plan.output.join("state");
    let logs_dir = plan.output.join("logs");
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
    let mut previous: Option<RunState> = RunState::load(&state_path);
    let mut files = if let Some(ref rs) = previous {
        rs.files.clone()
    } else {
        Vec::new()
    };
    let mut already_done = std::collections::HashSet::new();
    for f in &files {
        if f.status == Status::Extracted {
            already_done.insert(f.path.clone());
        }
    }

    // Collect archives to process
    let mut archives = Vec::new();
    for entry in WalkDir::new(&plan.input).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if !p.is_file() { continue; }
        let family = detect_family(&p);
        if matches!(family, ArchiveFamily::Unknown) { continue; }
        archives.push(p.to_path_buf());
    }

    let pb = ProgressBar::new(archives.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
        .unwrap()
        .progress_chars("#>- "));

    for path in archives {
        let path_str = path.display().to_string();
        if already_done.contains(&path_str) {
            pb.inc(1);
            pb.set_message(format!("skip {}", path.file_name().unwrap().to_string_lossy()));
            continue;
        }
        let family = detect_family(&path);
        let engine: Box<dyn Engine> = match family {
            ArchiveFamily::Rar => Box::new(RarEngine),
            ArchiveFamily::Zip => Box::new(ZipEngine),
            _ => continue,
        };
        let dest = target_dir(plan, &path)?;
        let state_entry = match engine.extract(&path, &dest) {
            Ok(_) => {
                pb.inc(1);
                pb.set_message(format!("OK {}", path.file_name().unwrap().to_string_lossy()));
                FileState { path: path_str.clone(), status: Status::Extracted }
            }
            Err(e) => {
                pb.inc(1);
                pb.set_message(format!("ERR {}", path.file_name().unwrap().to_string_lossy()));
                FileState { path: path_str.clone(), status: Status::Error }
            }
        };
        files.push(state_entry);
    }
    pb.finish_with_message("hecho");

    // Save run state
    let run_state = RunState { id: run_id.clone(), files };
    run_state.save(&state_path).map_err(|e| e.to_string())?;

    // Append history only on first run
    if previous.is_none() {
        let entry = format!("{{\"id\":\"{}\",\"input\":\"{}\",\"output\":\"{}\",\"ts\":{}}}", 
            run_id, plan.input.display(), plan.output.display(), 
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
        history::append_history(&history_path, &entry).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn target_dir(plan: &Plan, archive_path: &Path) -> Result<PathBuf, String> {
    let rel = archive_path.strip_prefix(&plan.input).map_err(|e| e.to_string())?;
    let stem = archive_path.file_stem().and_then(|s| s.to_str()).unwrap_or("archive");
    let base_name = format!("{stem}");
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
    Ok(candidate)
}

use crate::core::planner::Plan;
use crate::engines::detector::{detect_family, ArchiveFamily};
use jwalk::WalkDir;

pub fn print_diagnostics(plan: &Plan) {
    let mut total = 0usize;
    let mut compatible = 0usize;
    let mut incompatible = 0usize;
    for entry in WalkDir::new(&plan.input).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if !p.is_file() { continue; }
        total += 1;
        match detect_family(&p) {
            ArchiveFamily::Unknown => incompatible += 1,
            _ => compatible += 1,
        }
    }
    println!("Diagnóstico previo:");
    println!("  Archivos encontrados: {}", total);
    println!("  Compatibles: {}", compatible);
    println!("  No compatibles: {}", incompatible);
}

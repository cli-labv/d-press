mod cli;
mod core;
mod engines;
mod state;

use crate::cli::{adaptive_menu, args::Args, diagnostics, history_view, menus};
use crate::core::extractor::run;
use crate::core::planner::{OutputMode, Plan};
use clap::Parser;

fn main() {
    let args = Args::parse();
    crate::cli::banner_big::print_big_banner();
    if args.has_inputs() {
        // Modo no interactivo
        let input = args.input.unwrap();
        let output = args.output.unwrap();
        let mode = match args.mode.as_str() {
            "preserve" | "b" => OutputMode::Preserve,
            _ => OutputMode::Flat,
        };
        let plan = Plan::new(input, output, mode);
        diagnostics::print_diagnostics(&plan);
        if !args.yes && !menus::confirm_extract() {
            return;
        }
        if let Err(e) = run(&plan) {
            eprintln!("Error: {}", e);
        }
        return;
    }

    // Modo interactivo
    loop {
        let choice = adaptive_menu::main_menu_adaptive();
        match choice {
            "Procesar ./input" => {
                let (input, output) = menus::ask_input_output();
                let flat = menus::ask_mode();
                let mode = if flat {
                    OutputMode::Flat
                } else {
                    OutputMode::Preserve
                };
                let plan = Plan::new(input, output, mode);
                diagnostics::print_diagnostics(&plan);
                if args.yes || menus::confirm_extract() {
                    if let Err(e) = run(&plan) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            "Carpeta externa" => {
                println!("Función pendiente");
            }
            "Historial" => {
                let log_path = std::path::Path::new("registros/historial.jsonl");
                history_view::show_history(log_path);
            }
            "Reanudar" => {
                let (input, output) = menus::ask_input_output();
                let flat = menus::ask_mode();
                let mode = if flat {
                    OutputMode::Flat
                } else {
                    OutputMode::Preserve
                };
                let plan = Plan::new(input, output, mode);
                if let Err(e) = run(&plan) {
                    eprintln!("Error: {}", e);
                }
            }
            "Salir" => break,
            _ => {}
        }
    }
}

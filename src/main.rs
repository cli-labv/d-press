mod engines;
mod core;
mod state;
mod cli;

use std::fs;
use std::path::Path;
use crate::core::planner::{Plan, OutputMode};
use crate::core::extractor::run;
use crate::cli::{menus, diagnostics, args::Args, history_view, banner};
use clap::Parser;

fn main() {
    let args = Args::parse();
    let banner_config = crate::cli::banner::BannerConfig::from_args(&args);
    banner::print_banner(&banner_config);
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
        if !args.yes {
            let ok = dialoguer::Confirm::new()
                .with_prompt("¿Iniciar extracción?")
                .default(true)
                .interact()
                .unwrap();
            if !ok { return; }
        }
        if let Err(e) = run(&plan) {
            eprintln!("Error: {}", e);
        } else {
            println!("Proceso finalizado.");
        }
        return;
    }

    // Modo interactivo
    println!("d-press");
    loop {
        let choice = menus::main_menu();
        match choice {
            "Procesar carpeta ./input" => {
                let (input, output) = menus::ask_input_output();
                let flat = menus::ask_mode();
                let mode = if flat { OutputMode::Flat } else { OutputMode::Preserve };
                let plan = Plan::new(input, output, mode);
                diagnostics::print_diagnostics(&plan);
                let confirm = dialoguer::Confirm::new()
                    .with_prompt("¿Iniciar extracción?")
                    .default(true)
                    .interact()
                    .unwrap();
                if confirm {
                    if let Err(e) = run(&plan) {
                        eprintln!("Error: {}", e);
                    } else {
                        println!("Proceso finalizado.");
                    }
                }
            }
            "Seleccionar carpeta externa" => {
                println!("Función pendiente");
            }
            "Ver historial" => {
                let log_path = std::path::Path::new("logs/historial.jsonl");
                history_view::show_history(log_path);
            }
            "Reanudar corrida pendiente" => {
                let (input, output) = menus::ask_input_output();
                let flat = menus::ask_mode();
                let mode = if flat { OutputMode::Flat } else { OutputMode::Preserve };
                let plan = Plan::new(input, output, mode);
                println!("Reanudando...");
                if let Err(e) = run(&plan) {
                    eprintln!("Error: {}", e);
                } else {
                    println!("Reanudación finalizada.");
                }
            }
            "Salir" => break,
            _ => {}
        }
    }
}

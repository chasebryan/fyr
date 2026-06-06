use std::env;
use std::fs;
use std::process::ExitCode;

use fyr::{check_source, repl, run_source};

fn main() -> ExitCode {
    match run_cli() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

fn run_cli() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [] => repl::start().map_err(|error| error.to_string()),
        [_] => repl::start().map_err(|error| error.to_string()),
        [_, command] if command == "repl" => repl::start().map_err(|error| error.to_string()),
        [_, command] if command == "help" || command == "--help" || command == "-h" => {
            print_help();
            Ok(())
        }
        [_, command] if command == "version" || command == "--version" || command == "-V" => {
            println!("fyr 0.1.0");
            Ok(())
        }
        [_, command, path] if command == "check" => {
            let source = read_source(path)?;
            check_source(&source).map_err(|error| error.to_string())?;
            println!("{path}: ok");
            Ok(())
        }
        [_, command, path] if command == "run" => {
            let source = read_source(path)?;
            let result = run_source(&source).map_err(|error| error.to_string())?;
            for line in result.outputs {
                println!("{line}");
            }
            Ok(())
        }
        [_, unknown, ..] => Err(format!(
            "unknown command '{unknown}'. Run `fyr help` for usage."
        )),
    }
}

fn read_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {path}: {error}"))
}

fn print_help() {
    println!("Fyr programming language bootstrap");
    println!();
    println!("Usage:");
    println!("  fyr              Start the REPL");
    println!("  fyr repl         Start the REPL");
    println!("  fyr run <file>   Run a Fyr source file");
    println!("  fyr check <file> Parse-check a Fyr source file");
    println!("  fyr version      Print the Fyr version");
}

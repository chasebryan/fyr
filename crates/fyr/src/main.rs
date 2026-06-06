use std::env;
use std::fs;
use std::path::{Path, PathBuf};
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
        [_, command] if command == "doctor" => {
            print_doctor()?;
            Ok(())
        }
        [_, command, paths @ ..] if command == "check" => check_files(paths),
        [_, command, path] if command == "run" => {
            let source = read_source(path)?;
            let result = run_source(&source).map_err(|error| error.to_string())?;
            for line in result.outputs {
                println!("{line}");
            }
            Ok(())
        }
        [_, command, paths @ ..] if command == "test" => test_files(paths),
        [_, unknown, ..] => Err(format!(
            "unknown command '{unknown}'. Run `fyr help` for usage."
        )),
    }
}

fn check_files(paths: &[String]) -> Result<(), String> {
    let files = collect_source_paths(paths, "check")?;

    for path in files {
        let source = read_source_path(&path)?;
        check_source(&source).map_err(|error| error.to_string())?;
        println!("{}: ok", path.display());
    }

    Ok(())
}

fn test_files(paths: &[String]) -> Result<(), String> {
    let files = collect_source_paths(paths, "test")?;
    let file_count = files.len();

    for path in files {
        let source = read_source_path(&path)?;
        let result = run_source(&source).map_err(|error| error.to_string())?;
        for line in result.outputs {
            println!("{line}");
        }
        println!("{}: pass", path.display());
    }

    if file_count > 1 {
        println!("{file_count} test files passed");
    }

    Ok(())
}

fn collect_source_paths(paths: &[String], command: &str) -> Result<Vec<PathBuf>, String> {
    if paths.is_empty() {
        return Err(format!("fyr {command} expects at least one path"));
    }

    let mut files = Vec::new();
    for path in paths {
        collect_source_path(Path::new(path), &mut files)?;
    }

    files.sort();
    files.dedup();

    if files.is_empty() {
        return Err(format!("fyr {command} found no .fyr files"));
    }

    Ok(files)
}

fn collect_source_path(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }

    if path.is_dir() {
        collect_source_dir(path, files)?;
        return Ok(());
    }

    Err(format!("{} does not exist", path.display()))
}

fn collect_source_dir(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_source_dir(&path, files)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("fyr") {
            files.push(path);
        }
    }

    Ok(())
}

fn read_source(path: &str) -> Result<String, String> {
    read_source_path(Path::new(path))
}

fn read_source_path(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn print_help() {
    println!("Fyr programming language bootstrap");
    println!();
    println!("Usage:");
    println!("  fyr              Start the REPL");
    println!("  fyr repl         Start the REPL");
    println!("  fyr run <file>   Run a Fyr source file");
    println!("  fyr check <path...> Parse-check Fyr files or directories");
    println!("  fyr test <path...>  Run Fyr assertion files or directories");
    println!("  fyr doctor       Show command/install diagnostics");
    println!("  fyr version      Print the Fyr version");
}

fn print_doctor() -> Result<(), String> {
    let exe = env::current_exe().map_err(|error| format!("failed to locate fyr: {error}"))?;
    let cwd = env::current_dir().map_err(|error| format!("failed to read cwd: {error}"))?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| "failed to locate fyr binary directory".to_owned())?;
    let path = env::var_os("PATH").unwrap_or_default();
    let on_path = env::split_paths(&path).any(|entry| same_path(&entry, exe_dir));

    println!("fyr doctor");
    println!("  version: 0.1.0");
    println!("  executable: {}", exe.display());
    println!("  cwd: {}", cwd.display());
    println!("  binary directory on PATH: {on_path}");

    if !on_path {
        println!("  hint: add {} to PATH", exe_dir.display());
    }

    Ok(())
}

fn same_path(left: &Path, right: &Path) -> bool {
    match (left.canonicalize(), right.canonicalize()) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

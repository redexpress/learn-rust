use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let mut physical = false;

    for arg in args {
        match arg.as_str() {
            "-P" | "--physical" => physical = true,
            "-L" | "--logical" => physical = false,
            _ => {
                eprintln!("pwd: invalid option '{arg}'");
                return ExitCode::FAILURE;
            }
        }
    }

    let path = if physical {
        match fs::canonicalize(".") {
            Ok(path) => path,
            Err(err) => {
                eprintln!("pwd: {err}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        match env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                eprintln!("pwd: {err}");
                return ExitCode::FAILURE;
            }
        }
    };

    println!("{}", to_unix_path(&path));

    ExitCode::SUCCESS
}

pub fn to_unix_path(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");

    if s.len() >= 2 && s.as_bytes()[1] == b':' {
        let drive = s.chars().next().unwrap().to_ascii_lowercase();
        format!("/{}{}", drive, &s[2..])
    } else {
        s
    }
}
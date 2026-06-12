use std::fs;
use std::io::{self, Write};
use std::path::Path;

struct Options {
    all: bool,
    dirs_only: bool,
    max_depth: Option<usize>,
}

pub fn run(args: &[String]) -> i32 {
    let mut opts = Options {
        all: false,
        dirs_only: false,
        max_depth: None,
    };
    let mut dirs: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-a" => opts.all = true,
            "-d" => opts.dirs_only = true,
            "-L" => {
                if i + 1 >= args.len() {
                    eprintln!("tree: option '-L' requires an argument");
                    return 2;
                }
                i += 1;
                match args[i].parse::<usize>() {
                    Ok(v) => opts.max_depth = Some(v),
                    _ => {
                        eprintln!("tree: invalid level '{}'", args[i]);
                        return 2;
                    }
                }
            }
            "-h" | "--help" => {
                print_help();
                return 0;
            }
            "--" => {
                dirs.extend_from_slice(&args[i + 1..]);
                break;
            }
            a if a.starts_with("-L") => {
                let v = &a[2..];
                match v.parse::<usize>() {
                    Ok(v) => opts.max_depth = Some(v),
                    _ => {
                        eprintln!("tree: invalid level '{v}'");
                        return 2;
                    }
                }
            }
            _ => {
                if !arg.starts_with('-') {
                    dirs.push(arg.clone());
                }
            }
        }
        i += 1;
    }

    if dirs.is_empty() {
        dirs.push(".".to_string());
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut first = true;

    for dir in &dirs {
        if !first {
            let _ = writeln!(handle);
        }
        first = false;

        let path = Path::new(dir);
        if dirs.len() > 1 {
            let _ = writeln!(handle, "{dir}");
        }

        if path.is_dir() {
            print_tree(&mut handle, path, "", &opts, 0);
        } else {
            let _ = writeln!(handle, "{dir}");
        }
    }

    0
}

fn print_tree(
    w: &mut impl Write,
    path: &Path,
    prefix: &str,
    opts: &Options,
    depth: usize,
) {
    if let Some(max) = opts.max_depth {
        if depth > max {
            return;
        }
    }

    let mut entries: Vec<_> = match fs::read_dir(path) {
        Ok(rd) => rd.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };

    entries.sort_by(|a, b| {
        let a_name = a.file_name();
        let b_name = b.file_name();
        let a_hidden = a_name.to_string_lossy().starts_with('.');
        let b_hidden = b_name.to_string_lossy().starts_with('.');
        if a_hidden != b_hidden {
            return a_hidden.cmp(&b_hidden);
        }
        a_name.cmp(&b_name)
    });

    let visible: Vec<_> = entries
        .iter()
        .filter(|e| {
            let name = e.file_name();
            let name_str = name.to_string_lossy();
            if !opts.all && name_str.starts_with('.') {
                return false;
            }
            if opts.dirs_only && !e.path().is_dir() {
                return false;
            }
            true
        })
        .collect();

    for (idx, entry) in visible.iter().enumerate() {
        let is_last = idx == visible.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        let _ = writeln!(w, "{prefix}{connector}{name_str}");

        if entry.path().is_dir() {
            let ext = if is_last { "    " } else { "│   " };
            let new_prefix = format!("{prefix}{ext}");
            print_tree(w, &entry.path(), &new_prefix, opts, depth + 1);
        }
    }
}

fn print_help() {
    println!("Usage: tree [options] [directory]");
    println!();
    println!("Options:");
    println!("  -a            show hidden files");
    println!("  -d            list directories only");
    println!("  -L LEVEL      max display depth");
    println!("  -h, --help    this help");
}

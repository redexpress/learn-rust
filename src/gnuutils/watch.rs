use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(unix)]
fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

#[cfg(windows)]
fn default_shell() -> String {
    "powershell".to_string()
}

pub fn run(args: &[String]) -> i32 {
    let mut interval: f64 = 2.0;
    let mut no_title = false;
    let mut differences = false;
    let mut color = false;
    let mut beep = false;
    let mut errexit = false;
    let mut chgexit = false;
    let mut command_args: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-n" | "--interval" => {
                if i + 1 >= args.len() {
                    eprintln!("watch: option '{arg}' requires an argument");
                    return 2;
                }
                i += 1;
                match args[i].parse::<f64>() {
                    Ok(v) if v > 0.0 => interval = v,
                    _ => {
                        eprintln!("watch: invalid interval '{}'", args[i]);
                        return 2;
                    }
                }
            }
            "-d" | "--differences" => differences = true,
            "-c" | "--color" => color = true,
            "-t" | "--no-title" => no_title = true,
            "-b" | "--beep" => beep = true,
            "-e" | "--errexit" => errexit = true,
            "-g" | "--chgexit" => chgexit = true,
            "-h" | "--help" => {
                print_help();
                return 0;
            }
            "--" => {
                command_args.extend_from_slice(&args[i + 1..]);
                break;
            }
            a if a.starts_with("--interval=") => {
                let v = &a["--interval=".len()..];
                match v.parse::<f64>() {
                    Ok(v) if v > 0.0 => interval = v,
                    _ => {
                        eprintln!("watch: invalid interval '{v}'");
                        return 2;
                    }
                }
            }
            _ => {
                command_args.push(arg.clone());
            }
        }
        i += 1;
    }

    if command_args.is_empty() {
        eprintln!("watch: no command given");
        return 2;
    }

    let interval_dur = Duration::from_secs_f64(interval);
    let mut prev_lines: Option<Vec<Vec<u8>>> = None;
    let mut prev_height: usize = 0;
    let mut first = true;
    let mut changed = false;

    loop {
        let (cleaned, status) = run_command(&command_args, color);
        let cur_lines: Vec<Vec<u8>> = cleaned.split(|&b| b == b'\n').map(|s| s.to_vec()).collect();
        let cur_height = cur_lines.len();

        if first {
            let mut out = io::stdout();
            let _ = write!(out, "\x1B[2J\x1B[H");
            if !no_title {
                print_title(&command_args, interval);
            }
            if differences {
                print_diff_lines(&cur_lines, &[]);
            } else {
                for line in &cur_lines {
                    let _ = out.write_all(line);
                    let _ = out.write_all(b"\n");
                }
            }
            let _ = out.flush();
            prev_lines = Some(cur_lines);
            prev_height = cur_height;
            first = false;
        } else {
            let prev = prev_lines.as_deref().unwrap_or(&[]);
            changed = cur_lines != *prev;

            if changed {
                let mut out = io::stdout();
                if !no_title {
                    let _ = write!(out, "\x1B[H");
                    print_title(&command_args, interval);
                } else {
                    let _ = write!(out, "\x1B[H");
                }
                let title_offset = if no_title { 0 } else { 2 };
                for i in 0..cur_height {
                    let new_line = cur_lines.get(i).map(|v| v.as_slice()).unwrap_or(b"");
                    let old_line = prev.get(i).map(|v| v.as_slice()).unwrap_or(b"");
                    if new_line == old_line {
                        continue;
                    }
                    let _ = write!(out, "\x1B[{};1H\x1B[K", i + 1 + title_offset);
                    if differences {
                        let _ = out.write_all(b"\x1B[33m");
                        let _ = out.write_all(new_line);
                        let _ = out.write_all(b"\x1B[0m");
                    } else {
                        let _ = out.write_all(new_line);
                    }
                }
                for i in cur_height..prev_height {
                    let _ = write!(out, "\x1B[{};1H\x1B[K", i + 1 + title_offset);
                }
                let _ = write!(out, "\x1B[{};1H", cur_height + title_offset + 1);
                let _ = out.flush();
            }
            prev_lines = Some(cur_lines);
            prev_height = cur_height;
        }

        if status != 0 {
            eprintln!("[exit {status}]");
            if beep {
                eprint!("\x07");
            }
            if errexit {
                return status;
            }
        }

        if chgexit && changed && !first {
            return 0;
        }

        std::thread::sleep(interval_dur);
    }
}

fn print_title(cmd: &[String], interval: f64) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cmdline = shell_join(cmd);
    let _ = writeln!(io::stdout(), "Every {interval}s: {cmdline}    {now}");
    let _ = writeln!(io::stdout());
}

fn shell_join(args: &[String]) -> String {
    let mut out = String::new();
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        if a.chars().any(char::is_whitespace) {
            out.push('\'');
            out.push_str(&a.replace('\'', "'\\''"));
            out.push('\'');
        } else {
            out.push_str(a);
        }
    }
    out
}

fn run_command(args: &[String], color: bool) -> (Vec<u8>, i32) {
    let mut cmd = Command::new(default_shell());
    #[cfg(windows)]
    {
        cmd.arg("-NoLogo").arg("-NoProfile").arg("-Command");
    }
    #[cfg(unix)]
    {
        cmd.arg("-c");
    }
    cmd.arg(shell_join(args))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if !color {
        cmd.env("TERM", "dumb");
    }
    match cmd.output() {
        Ok(out) => {
            let mut combined = out.stdout;
            combined.extend_from_slice(&out.stderr);
            let cleaned = strip_ansi(&combined, !color);
            (cleaned, out.status.code().unwrap_or(1))
        }
        Err(e) => {
            let msg = format!("watch: failed to spawn: {e}\n");
            (msg.into_bytes(), 127)
        }
    }
}

fn strip_ansi(bytes: &[u8], strip: bool) -> Vec<u8> {
    if !strip {
        return bytes.to_vec();
    }
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
            i += 2;
            while i < bytes.len() && !(0x40..=0x7E).contains(&bytes[i]) {
                i += 1;
            }
            i += 1;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn print_diff_lines(current: &[Vec<u8>], prev: &[Vec<u8>]) {
    let mut out = io::stdout();
    for (i, line) in current.iter().enumerate() {
        let changed = prev.get(i).map(|p| p.as_slice() != line.as_slice()).unwrap_or(true);
        if changed {
            let _ = out.write_all(b"\x1B[33m");
            let _ = out.write_all(line);
            let _ = out.write_all(b"\x1B[0m");
        } else {
            let _ = out.write_all(line);
        }
        let _ = out.write_all(b"\n");
    }
}

fn print_help() {
    println!("Usage: watch [options] command");
    println!();
    println!("Options:");
    println!("  -n, --interval SECS     seconds between updates (default 2)");
    println!("  -d, --differences       highlight changed lines");
    println!("  -c, --color             interpret ANSI color sequences");
    println!("  -t, --no-title          suppress the header");
    println!("  -b, --beep              beep if command has a non-zero exit");
    println!("  -e, --errexit           exit if command has a non-zero exit");
    println!("  -g, --chgexit           exit when output changes");
    println!("  -h, --help              this help");
}

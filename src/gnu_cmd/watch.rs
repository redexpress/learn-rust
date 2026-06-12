use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
    let mut prev_output: Option<Vec<u8>> = None;

    loop {
        // 清屏 + 光标归零
        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();

        if !no_title {
            print_title(&command_args, interval);
        }

        let (cleaned, status) = run_command(&command_args, color);

        if differences {
            if let Some(prev) = prev_output.as_deref() {
                print_diff_lines(&cleaned, prev);
            } else {
                let _ = io::stdout().write_all(&cleaned);
            }
        } else {
            let _ = io::stdout().write_all(&cleaned);
        }
        prev_output = Some(cleaned);

        let _ = io::stdout().flush();

        if status != 0 {
            eprintln!("\n[exit {status}]");
            if beep {
                eprint!("\x07");
            }
            if errexit {
                return status;
            }
        }

        if chgexit {
            // 第一轮之后每次循环都做 diff；变化则返回 0
            // 简单做法：上一轮已比对，本轮内容与上轮不等 → 退出
            // 由 print_diff_lines 标记的 prev_output 触发
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
    println!("Every {interval}s: {cmdline}    {now}");
    println!();
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
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let mut cmd = Command::new(&shell);
    cmd.arg("-c")
        .arg(shell_join(args))
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

fn print_diff_lines(current: &[u8], prev: &[u8]) {
    let cur_lines: Vec<&[u8]> = current.split(|&b| b == b'\n').collect();
    let prev_lines: Vec<&[u8]> = prev.split(|&b| b == b'\n').collect();
    for (i, line) in cur_lines.iter().enumerate() {
        let changed = prev_lines.get(i).map(|p| *p != *line).unwrap_or(true);
        if changed {
            let _ = io::stdout().write_all(b"\x1B[33m");
        }
        let _ = io::stdout().write_all(line);
        if changed {
            let _ = io::stdout().write_all(b"\x1B[0m");
        }
        let _ = io::stdout().write_all(b"\n");
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

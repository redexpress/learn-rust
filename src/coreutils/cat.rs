use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::ExitCode;

struct Options {
    number_all: bool,
    number_nonblank: bool,
    squeeze_blank: bool,
    show_ends: bool,
    show_tabs: bool,
    show_nonprinting: bool,
}

impl Options {
    fn new() -> Self {
        Self {
            number_all: false,
            number_nonblank: false,
            squeeze_blank: false,
            show_ends: false,
            show_tabs: false,
            show_nonprinting: false,
        }
    }
}

pub fn run(args: &[String]) -> ExitCode {
    let mut opts = Options::new();
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--" {
            i += 1;
            while i < args.len() {
                files.push(args[i].clone());
                i += 1;
            }
            break;
        }

        if !arg.starts_with('-') || arg == "-" || arg.len() == 1 {
            files.push(arg.clone());
            i += 1;
            continue;
        }

        let flags = &arg[1..];
        let mut parsed = false;

        for c in flags.chars() {
            match c {
                'n' => {
                    opts.number_all = true;
                    parsed = true;
                }
                'b' => {
                    opts.number_nonblank = true;
                    parsed = true;
                }
                's' => {
                    opts.squeeze_blank = true;
                    parsed = true;
                }
                'E' => {
                    opts.show_ends = true;
                    parsed = true;
                }
                'T' => {
                    opts.show_tabs = true;
                    parsed = true;
                }
                'A' => {
                    opts.show_nonprinting = true;
                    opts.show_ends = true;
                    opts.show_tabs = true;
                    parsed = true;
                }
                'e' => {
                    opts.show_nonprinting = true;
                    opts.show_ends = true;
                    parsed = true;
                }
                't' => {
                    opts.show_nonprinting = true;
                    opts.show_tabs = true;
                    parsed = true;
                }
                'v' => {
                    opts.show_nonprinting = true;
                    parsed = true;
                }
                'u' => {
                    parsed = true;
                }
                _ => {
                    eprintln!("cat: invalid option -- '{c}'");
                    return ExitCode::FAILURE;
                }
            }
        }

        if !parsed {
            files.push(arg.clone());
        }

        i += 1;
    }

    if files.is_empty() {
        files.push("-".to_string());
    }

    let mut exit_code = ExitCode::SUCCESS;
    let mut line_number: u64 = 1;
    let mut prev_blank = false;

    for file in &files {
        if file == "-" {
            let stdin = io::stdin();
            if cat_reader(
                stdin.lock(),
                &opts,
                &mut line_number,
                &mut prev_blank,
                "(standard input)",
            )
            .is_err()
            {
                exit_code = ExitCode::FAILURE;
            }
        } else {
            match File::open(file) {
                Ok(f) => {
                    if cat_reader(
                        BufReader::new(f),
                        &opts,
                        &mut line_number,
                        &mut prev_blank,
                        file,
                    )
                    .is_err()
                    {
                        exit_code = ExitCode::FAILURE;
                    }
                }
                Err(e) => {
                    eprintln!("cat: {file}: {e}");
                    exit_code = ExitCode::FAILURE;
                }
            }
        }
    }

    exit_code
}

fn cat_reader<R: Read>(
    reader: R,
    opts: &Options,
    line_number: &mut u64,
    prev_blank: &mut bool,
    _filename: &str,
) -> io::Result<()> {
    let buf_reader = BufReader::new(reader);
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let is_blank = line.is_empty();

        if opts.squeeze_blank && is_blank {
            if *prev_blank {
                continue;
            }
            *prev_blank = true;
        } else {
            *prev_blank = false;
        }

        let show_number = if opts.number_nonblank {
            !is_blank
        } else {
            opts.number_all
        };

        if show_number {
            write!(handle, "{:>6}\t", line_number)?;
            *line_number += 1;
        }

        let mut buf = String::with_capacity(line.len() + 16);

        for ch in line.chars() {
            if opts.show_nonprinting {
                match ch {
                    '\t' if opts.show_tabs => {
                        buf.push('^');
                        buf.push('I');
                    }
                    '\t' => buf.push('\t'),
                    c if c.is_control() && c != '\n' && c != '\r' => {
                        buf.push('^');
                        buf.push(char::from_u32(c as u32 ^ 0x40).unwrap_or('?'));
                    }
                    c if (c as u32) >= 0x80 => {
                        if (c as u32) < 0xA0 {
                            buf.push_str("M-^");
                            buf.push(char::from_u32((c as u32 - 0x80) ^ 0x40).unwrap_or('?'));
                        } else {
                            buf.push('M');
                            buf.push('-');
                            buf.push(char::from_u32(c as u32 - 0x80).unwrap_or('?'));
                        }
                    }
                    c => buf.push(c),
                }
            } else {
                match ch {
                    '\t' if opts.show_tabs => {
                        buf.push('^');
                        buf.push('I');
                    }
                    c => buf.push(c),
                }
            }
        }

        if opts.show_ends {
            buf.push('$');
        }

        writeln!(handle, "{buf}")?;
    }

    Ok(())
}

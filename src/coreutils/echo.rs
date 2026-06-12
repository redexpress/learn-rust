use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let mut newline = true;
    let mut escape = false;

    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];

        if !arg.starts_with('-') || arg.len() <= 1 {
            break;
        }

        let flags = &arg[1..];

        if flags.chars().all(|c| matches!(c, 'n' | 'e' | 'E')) {
            for c in flags.chars() {
                match c {
                    'n' => newline = false,
                    'e' => escape = true,
                    'E' => escape = false,
                    _ => {}
                }
            }

            index += 1;
        } else {
            break;
        }
    }

    let mut first = true;

    for arg in &args[index..] {
        if !first {
            print!(" ");
        }

        if escape {
            print!("{}", unescape(arg));
        } else {
            print!("{arg}");
        }

        first = false;
    }

    if newline {
        println!();
    }

    ExitCode::SUCCESS
}

fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            result.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => result.push('\n'),
            Some('t') => result.push('\t'),
            Some('r') => result.push('\r'),
            Some('\\') => result.push('\\'),
            Some(c) => {
                result.push('\\');
                result.push(c);
            }
            None => result.push('\\'),
        }
    }

    result
}
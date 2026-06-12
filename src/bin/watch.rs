use std::env;

use lrust::gnu_cmd::watch;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = watch::run(&args);
    if code != 0 {
        std::process::exit(code);
    }
}

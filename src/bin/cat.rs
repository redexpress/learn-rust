use std::env;
use std::process::ExitCode;

use lrust::coreutils::cat;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    cat::run(&args)
}

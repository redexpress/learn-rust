use std::env;
use std::process::ExitCode;

use lrust::coreutils::pwd;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    pwd::run(&args)
}
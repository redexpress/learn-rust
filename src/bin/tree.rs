use std::env;

use lrust::gnuutils::tree;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let code = tree::run(&args);
    if code != 0 {
        std::process::exit(code);
    }
}

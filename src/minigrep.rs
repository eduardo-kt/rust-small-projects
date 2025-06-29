use std::env;

pub fn run_minigrep() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
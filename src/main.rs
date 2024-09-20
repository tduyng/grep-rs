use std::env;
use std::process;

use grep_starter_rust::cli::Config;
use grep_starter_rust::run;

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

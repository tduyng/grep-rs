pub mod cli;

pub fn run(config: cli::Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if match_pattern(&input, &config.pattern) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    match pattern {
        _ if pattern.chars().count() == 1 => input.contains(pattern),
        "\\d" => input.chars().any(|c| c.is_digit(10)),
        _ => unimplemented!("Pattern '{}' is not implemented yet.", pattern),
    }
}

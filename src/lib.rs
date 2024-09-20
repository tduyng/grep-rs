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

pub fn match_pattern(input: &str, pattern: &str) -> bool {
    if is_char_group(pattern) {
        let chars: Vec<char> = pattern[1..pattern.len() - 1].chars().collect();
        input.chars().any(|c| chars.contains(&c))
    } else {
        match pattern {
            _ if pattern.chars().count() == 1 => input.contains(pattern),
            "\\d" => input.chars().any(|c| c.is_digit(10)),
            "\\w" => input.chars().any(|c| c.is_alphanumeric() || c == '_'),
            _ => {
                eprintln!("Unhandled pattern: {}", pattern);
                false
            }
        }
    }
}

fn is_char_group(pattern: &str) -> bool {
    pattern.starts_with('[') && pattern.ends_with(']')
}

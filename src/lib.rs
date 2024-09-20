use std::{iter::Peekable, str::Chars};

pub mod cli;

pub fn run(config: cli::Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if match_pattern(&input.trim(), &config.pattern) {
        println!("{}", input.trim());
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

pub fn match_pattern(input: &str, pattern: &str) -> bool {
    let mut input_chars = input.chars().peekable();
    let mut pattern_chars = pattern.chars().peekable();

    if is_start_of_line_anchor(&mut pattern_chars) {
        return match_start_of_line(&mut input_chars, &mut pattern_chars);
    }

    while input_chars.peek().is_some() {
        if match_at_position(&mut input_chars.clone(), &mut pattern.chars().peekable()) {
            return true;
        }
        input_chars.next();
    }

    false
}

fn is_start_of_line_anchor(pattern_chars: &mut Peekable<Chars>) -> bool {
    if let Some('^') = pattern_chars.peek() {
        pattern_chars.next(); // Consume the `^`
        true
    } else {
        false
    }
}

fn match_start_of_line(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
) -> bool {
    for pat_char in pattern_chars {
        if let Some(input_char) = input_chars.next() {
            if pat_char != input_char {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn match_at_position(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
) -> bool {
    while let Some(p) = pattern_chars.next() {
        if !match_single(input_chars, pattern_chars, p) {
            return false;
        }
    }

    true
}

fn match_single(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
    p: char,
) -> bool {
    match p {
        '\\' => match_class(input_chars, pattern_chars),
        '[' => match_bracket_class(input_chars, pattern_chars),
        c if c.is_whitespace() => match_whitespace(input_chars),
        c => match_literal(input_chars, c),
    }
}

fn match_class(input_chars: &mut Peekable<Chars>, pattern_chars: &mut Peekable<Chars>) -> bool {
    match pattern_chars.next() {
        Some('d') => match_digit(input_chars),
        Some('w') => match_alphanumeric(input_chars),
        Some('s') => match_whitespace(input_chars),
        _ => false,
    }
}
fn match_bracket_class(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
) -> bool {
    let negated = match pattern_chars.peek() {
        Some('^') => {
            pattern_chars.next(); // Consume '^'
            true
        }
        _ => false,
    };

    let mut matched = false;
    while let Some(c) = pattern_chars.next() {
        if c == ']' {
            break; // End of class
        }

        if let Some(input_c) = input_chars.peek() {
            if *input_c == c {
                matched = true;
                input_chars.next(); // Consume input character if it matches
            }
        }
    }

    if negated {
        !matched // Negate the result if the class was negated
    } else {
        matched
    }
}

fn match_digit(input_chars: &mut Peekable<Chars>) -> bool {
    if let Some(c) = input_chars.peek() {
        if c.is_digit(10) {
            input_chars.next(); // Consume the digit
            return true;
        }
    }
    false
}

fn match_alphanumeric(input_chars: &mut Peekable<Chars>) -> bool {
    if let Some(c) = input_chars.peek() {
        if c.is_alphanumeric() || *c == '_' {
            input_chars.next(); // Consume the alphanumeric character
            return true;
        }
    }
    false
}

fn match_whitespace(input_chars: &mut Peekable<Chars>) -> bool {
    while let Some(c) = input_chars.peek() {
        if c.is_whitespace() {
            input_chars.next(); // Consume all whitespace characters
        } else {
            break;
        }
    }
    true
}

fn match_literal(input_chars: &mut Peekable<Chars>, expected: char) -> bool {
    if let Some(c) = input_chars.peek() {
        if *c == expected {
            input_chars.next(); // Consume the matching literal character
            return true;
        }
    }
    false
}

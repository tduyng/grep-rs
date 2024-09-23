use std::{
    io::{self, BufRead},
    iter::Peekable,
    str::Chars,
};

pub mod cli;

pub fn run(config: cli::Config) -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line?;
        if match_pattern(&line, &config.pattern) {
            println!("{}", line);
            std::process::exit(0);
        }
    }

    std::process::exit(1);
}

fn match_pattern(input: &str, pattern: &str) -> bool {
    let mut input_chars = input.chars().peekable();
    let mut pattern_chars = pattern.chars().peekable();

    if let Some('^') = pattern_chars.peek() {
        return match_start(&mut input_chars, &mut pattern_chars);
    }

    if pattern.ends_with('$') {
        return match_end(&mut input_chars, pattern);
    }

    while input_chars.peek().is_some() {
        if match_at_position(&mut input_chars.clone(), &mut pattern.chars().peekable()) {
            return true;
        }
        input_chars.next();
    }

    false
}

fn match_start(input_chars: &mut Peekable<Chars>, pattern_chars: &mut Peekable<Chars>) -> bool {
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

fn match_end(input_chars: &mut Peekable<Chars>, pattern: &str) -> bool {
    let trimmed_pattern = &pattern[..pattern.len() - 1]; // Remove the `$`
    let pattern_chars = trimmed_pattern.chars().peekable();

    for pat_char in pattern_chars {
        if let Some(input_char) = input_chars.next() {
            if pat_char != input_char {
                return false;
            }
        } else {
            return false;
        }
    }

    input_chars.peek().is_none()
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
        '\\' => match_escape_class(input_chars, pattern_chars),
        '[' => match_bracket_class(input_chars, pattern_chars),
        '+' => match_one_or_more(input_chars, pattern_chars),
        c => match_literal(input_chars, c),
    }
}

fn match_escape_class(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
) -> bool {
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
            pattern_chars.next();
            true
        }
        _ => false,
    };

    let mut matched = false;
    while let Some(c) = pattern_chars.next() {
        if c == ']' {
            break;
        }
        if let Some(&input_c) = input_chars.peek() {
            if input_c == c {
                matched = true;
                input_chars.next();
            }
        }
    }

    if negated {
        !matched
    } else {
        matched
    }
}

fn match_one_or_more(
    input_chars: &mut Peekable<Chars>,
    pattern_chars: &mut Peekable<Chars>,
) -> bool {
    if let Some(&first_char) = pattern_chars.peek() {
        pattern_chars.next();

        if let Some(input_char) = input_chars.next() {
            if input_char != first_char {
                return false;
            }

            while let Some(&next_char) = input_chars.peek() {
                if next_char == first_char {
                    input_chars.next();
                } else {
                    break;
                }
            }

            return true;
        }
    }
    false
}

fn match_digit(input_chars: &mut Peekable<Chars>) -> bool {
    if let Some(c) = input_chars.peek() {
        if c.is_ascii_digit() {
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

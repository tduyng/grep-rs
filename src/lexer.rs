use std::char;

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(char),
    Wildcard,
    StartAnchor,
    EndAnchor,
    GroupStart,
    GroupEnd,
    Alternation,
    Quantifier(Quantifier),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Quantifier {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

pub fn tokenize(pattern: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '^' => tokens.push(Token::StartAnchor),
            '$' => tokens.push(Token::EndAnchor),
            '.' => tokens.push(Token::Wildcard),
            '(' => tokens.push(Token::GroupStart),
            ')' => tokens.push(Token::GroupEnd),
            '|' => tokens.push(Token::Alternation),
            '*' => tokens.push(Token::Quantifier(Quantifier::ZeroOrMore)),
            '+' => tokens.push(Token::Quantifier(Quantifier::OneOrMore)),
            '?' => tokens.push(Token::Quantifier(Quantifier::ZeroOrOne)),
            _ => tokens.push(Token::Literal(ch)),
        }
    }

    tokens.push(Token::EOF);
    tokens
}

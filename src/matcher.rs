use crate::parser::AstNode;
use std::{iter::Peekable, str::Chars};

pub fn match_pattern(ast: &Vec<AstNode>, text: &str) -> bool {
    let mut chars = text.chars().peekable();
    for node in ast {
        if !match_ast_node(node, &mut chars) {
            return false;
        }
    }
    true
}

fn match_ast_node(node: &AstNode, chars: &mut Peekable<Chars>) -> bool {
    match node {
        AstNode::Literal(ch) => {
            if let Some(next_char) = chars.peek() {
                if next_char == ch {
                    chars.next();
                    return true;
                }
            }
            false
        }
        AstNode::Wildcard => chars.next().is_some(),
        AstNode::StartAnchor => chars.peek().is_none(),
        AstNode::EndAnchor => chars.next().is_none(),
        AstNode::Group(group) => {
            for child in group {
                if !match_ast_node(child, chars) {
                    return false;
                }
            }
            true
        }
        AstNode::Alternation(left, right) => {
            let mut temp_chars = chars.clone();
            if left.iter().all(|n| match_ast_node(n, &mut temp_chars)) {
                *chars = temp_chars;
                return true;
            }

            temp_chars = chars.clone();
            if right.iter().all(|n| match_ast_node(n, &mut temp_chars)) {
                *chars = temp_chars;
                return true;
            }
            false
        }
        _ => unimplemented!(),
    }
}

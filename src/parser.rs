use crate::lexer::{Quantifier, Token};

#[derive(Debug)]
pub enum AstNode {
    Literal(char),
    Wildcard,
    Group(Vec<AstNode>),
    Alternation(Vec<AstNode>, Vec<AstNode>),
    Repetition(Box<AstNode>, Quantifier),
    StartAnchor,
    EndAnchor,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn next_token(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Result<Vec<AstNode>, String> {
        let mut ast = vec![];
        while self.current_token() != &Token::EOF {
            ast.push(self.parse_primary()?);
        }
        Ok(ast)
    }

    fn parse_primary(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Literal(ch) => {
                let ch = *ch;
                self.next_token();
                Ok(AstNode::Literal(ch))
            }
            Token::Wildcard => {
                self.next_token();
                Ok(AstNode::Wildcard)
            }
            Token::GroupStart => self.parse_group(),
            Token::Alternation => self.parse_alternation(),
            Token::Quantifier(q) => Err(format!("Unexpected quantifier {:?}", q)),
            Token::StartAnchor => {
                self.next_token();
                Ok(AstNode::StartAnchor)
            }
            Token::EndAnchor => {
                self.next_token();
                Ok(AstNode::EndAnchor)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current_token())),
        }
    }

    fn parse_group(&mut self) -> Result<AstNode, String> {
        self.next_token();
        let mut group = vec![];
        while self.current_token() != &Token::GroupEnd {
            group.push(self.parse_primary()?);
        }
        self.next_token();
        Ok(AstNode::Group(group))
    }

    fn parse_alternation(&mut self) -> Result<AstNode, String> {
        let mut left = Vec::new();

        // Parse the left side up to the first alternation or the end of the group
        while self.current_token() != &Token::Alternation
            && self.current_token() != &Token::GroupEnd
            && self.current_token() != &Token::EOF
        {
            left.push(self.parse_primary()?);
        }

        // If there's no alternation, return just the left side
        if self.current_token() != &Token::Alternation {
            return Ok(AstNode::Group(left));
        }

        // Consume '|'
        self.next_token();

        // Parse the right side until the end of the group or another alternation
        let mut right = Vec::new();
        while self.current_token() != &Token::GroupEnd && self.current_token() != &Token::EOF {
            right.push(self.parse_primary()?);
        }

        Ok(AstNode::Alternation(left, right))
    }
}

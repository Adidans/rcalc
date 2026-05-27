use thiserror::Error;

pub struct Lexer<'a> {
    source: &'a str,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer { source, current: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.current += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn number(&mut self, first: char) -> Result<Token, LexError> {
        let mut str = String::new();
        str.push(first);

        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            str.push(self.advance().unwrap());
        }
        Ok(Token::Literal(str.parse().unwrap()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        let ch = self.advance()?;
        let token = match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '+' => Token::Operator(Operator::Plus),
            '-' => Token::Operator(Operator::Minus),
            '*' => Token::Operator(Operator::Star),
            '/' => Token::Operator(Operator::Slash),
            c if c.is_ascii_digit() => return Some(self.number(c)),
            _ => return Some(Err(LexError::UnexpectedCharacter(ch))),
        };
        Some(Ok(token))
    }
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("Unexpected character {0}")]
    UnexpectedCharacter(char),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Literal(f64),
    Operator(Operator),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

fn get_precedence_value(op: &Token) -> u8 {
    match op {
        Token::Operator(Operator::Minus) | Token::Operator(Operator::Plus) => 1,
        Token::Operator(Operator::Star) | Token::Operator(Operator::Slash) => 2,
        Token::LParen | Token::RParen => 3,
        _ => unreachable!(),
    }
}

pub fn convert_to_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut out = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Literal(_) => out.push(token),
            Token::LParen => op_stack.push(token),
            Token::RParen => {
                while op_stack.last().is_some_and(|t| *t != Token::LParen) {
                    if let Some(op) = op_stack.pop() {
                        out.push(op);
                    }
                }
                op_stack.pop();
            }
            Token::Operator(_) => {
                while op_stack.last().is_some_and(|t| {
                    *t != Token::LParen && get_precedence_value(t) >= get_precedence_value(&token)
                }) {
                    if let Some(op) = op_stack.pop() {
                        out.push(op);
                    }
                }
                op_stack.push(token);
            }
        }
    }

    while let Some(op) = op_stack.pop() {
        out.push(op);
    }

    out
}

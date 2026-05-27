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
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
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

#[derive(Debug)]
pub enum Token {
    Literal(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

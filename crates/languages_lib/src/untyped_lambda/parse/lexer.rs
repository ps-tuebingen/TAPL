use std::collections::VecDeque;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lambda,
    Char(char),
    Dot,
    Space,
    ParensO,
    ParensC,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Lambda => f.write_str("\\"),
            Token::Char(c) => write!(f, "{c}"),
            Token::Dot => f.write_str("."),
            Token::Space => f.write_str(" "),
            Token::ParensO => f.write_str("("),
            Token::ParensC => f.write_str(")"),
        }
    }
}

fn consume_spaces(s: &mut String) {
    if s.is_empty() {
        return;
    }
    let next_char = s.remove(0);
    if next_char == ' ' || next_char == '\n' {
        consume_spaces(s)
    } else {
        s.insert(0, next_char)
    }
}

fn get_next(s: &mut String) -> Option<Token> {
    if s.is_empty() {
        return None;
    }

    match s.remove(0) {
        '\\' => Some(Token::Lambda),
        '.' => Some(Token::Dot),
        '(' => Some(Token::ParensO),
        ')' => Some(Token::ParensC),
        ' ' | '\n' => {
            consume_spaces(s);
            Some(Token::Space)
        }
        c => Some(Token::Char(c)),
    }
}

pub fn lex(s: &mut String) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    while let Some(tok) = get_next(s) {
        tokens.push_back(tok);
    }
    tokens
}

#[cfg(test)]
mod lexer_tests {
    use super::{lex, Token};
    #[test]
    fn tokenize_id() {
        let result = lex(&mut "\\x.x".to_owned());
        let expected = vec![
            Token::Lambda,
            Token::Char('x'),
            Token::Dot,
            Token::Char('x'),
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn tokenize_if() {
        let result = lex(&mut "\\l.\\m.l m n".to_owned());
        let expected = vec![
            Token::Lambda,
            Token::Char('l'),
            Token::Dot,
            Token::Lambda,
            Token::Char('m'),
            Token::Dot,
            Token::Char('l'),
            Token::Space,
            Token::Char('m'),
            Token::Space,
            Token::Char('n'),
        ];
        assert_eq!(result, expected)
    }
}

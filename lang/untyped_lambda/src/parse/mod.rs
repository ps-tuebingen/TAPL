use super::terms::{Term, Var};
use common::Parse;
use std::collections::VecDeque;

pub mod errors;
pub mod lexer;

use errors::Error;
use lexer::{lex, Token};

impl Parse for Term {
    type Err = Error;
    fn parse(mut input: String) -> Result<Self, Self::Err> {
        parse(&mut input)
    }
}

fn split_parensc(
    tokens: &mut VecDeque<Token>,
) -> Result<(VecDeque<Token>, VecDeque<Token>), Error> {
    let mut front = VecDeque::new();
    let mut open_parens = 0;

    let mut next_token = tokens.pop_front();
    loop {
        match next_token {
            None => return Err(Error::UnexpectedEOI),
            Some(Token::ParensO) => {
                open_parens += 1;
                front.push_back(Token::ParensO);
            }
            Some(Token::ParensC) => {
                if open_parens == 0 {
                    let back = tokens.clone();
                    tokens.clear();
                    return Ok((front, back));
                } else {
                    open_parens -= 1;
                    front.push_back(Token::ParensC)
                }
            }
            Some(tok) => front.push_back(tok),
        }
        next_token = tokens.pop_front();
    }
}

fn check_next(tokens: &mut VecDeque<Token>, expected: &Token) -> Result<Token, Error> {
    match tokens.pop_front() {
        None => Err(Error::UnexpectedEOI),
        Some(tok) => {
            if tok == *expected {
                Ok(tok)
            } else {
                Err(Error::UnexpectedToken(tok))
            }
        }
    }
}

fn parse_var(tokens: &mut VecDeque<Token>) -> Result<Var, Error> {
    let mut var = String::new();
    let mut next_token = tokens.pop_front();
    loop {
        match next_token {
            None => {
                return if var.is_empty() {
                    Err(Error::UnexpectedEOI)
                } else {
                    Ok(var)
                }
            }
            Some(Token::Char(c)) => var.push(c),
            Some(tok) => {
                tokens.push_front(tok);
                break;
            }
        };
        next_token = tokens.pop_front();
    }
    if var.is_empty() {
        Err(Error::UnexpectedEOI)
    } else {
        Ok(var)
    }
}

fn skip_spaces(tokens: &mut VecDeque<Token>) {
    if tokens.is_empty() {
        return;
    }
    let next_tok = tokens.pop_front();
    match next_tok {
        Some(Token::Space) => skip_spaces(tokens),
        Some(tok) => tokens.push_front(tok),
        None => (),
    }
}

fn parse_lambda(tokens: &mut VecDeque<Token>) -> Result<Term, Error> {
    skip_spaces(tokens);
    let var = parse_var(tokens)?;
    skip_spaces(tokens);
    check_next(tokens, &Token::Dot)?;
    skip_spaces(tokens);
    let body = parse_tokens(tokens)?;
    skip_spaces(tokens);

    let lambda_term = Term::Lambda(var, Box::new(body));
    if tokens.is_empty() {
        Ok(lambda_term)
    } else {
        let next_t = parse_tokens(tokens)?;
        Ok(Term::App(Box::new(lambda_term), Box::new(next_t)))
    }
}

fn parse_tokens(tokens: &mut VecDeque<Token>) -> Result<Term, Error> {
    match tokens.pop_front().ok_or(Error::UnexpectedEOI)? {
        Token::Lambda => parse_lambda(tokens),
        Token::Char(c) => {
            tokens.insert(0, Token::Char(c));
            let var = parse_var(tokens)?;
            skip_spaces(tokens);
            if tokens.is_empty() {
                Ok(Term::Var(var))
            } else {
                let rest_term = parse_tokens(tokens)?;
                Ok(Term::App(Box::new(Term::Var(var)), Box::new(rest_term)))
            }
        }
        Token::ParensO => {
            skip_spaces(tokens);
            let (mut front, mut back) = split_parensc(tokens)?;
            skip_spaces(&mut front);
            skip_spaces(&mut back);
            let inner_term = parse_tokens(&mut front)?;

            if back.is_empty() {
                Ok(inner_term)
            } else {
                let remaining_term = parse_tokens(&mut back)?;
                Ok(Term::App(Box::new(inner_term), Box::new(remaining_term)))
            }
        }
        Token::ParensC => Err(Error::ParenMismatch),
        Token::Dot => Err(Error::UnexpectedToken(Token::Dot)),
        Token::Space => Err(Error::UnexpectedToken(Token::Space)),
    }
}

pub fn parse(source: &mut String) -> Result<Term, Error> {
    let mut tokens = lex(source);
    parse_tokens(&mut tokens)
}

#[cfg(test)]
mod parser_tests {
    use super::parse;
    use crate::terms::Term;

    #[test]
    fn parse_id() {
        let result = parse(&mut "\\x.x".to_owned()).unwrap();
        let expected = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned())));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_true() {
        let result = parse(&mut "\\t.\\f.t".to_owned()).unwrap();
        let expected = Term::Lambda(
            "t".to_owned(),
            Box::new(Term::Lambda(
                "f".to_owned(),
                Box::new(Term::Var("t".to_owned())),
            )),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_false() {
        let result = parse(&mut "\\t.\\f.f".to_owned()).unwrap();
        let expected = Term::Lambda(
            "t".to_owned(),
            Box::new(Term::Lambda(
                "f".to_owned(),
                Box::new(Term::Var("f".to_owned())),
            )),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_if() {
        let result = parse(&mut "\\l.\\m.(l m) n".to_owned()).unwrap();
        let expected = Term::Lambda(
            "l".to_owned(),
            Box::new(Term::Lambda(
                "m".to_owned(),
                Box::new(Term::App(
                    Box::new(Term::App(
                        Box::new(Term::Var("l".to_owned())),
                        Box::new(Term::Var("m".to_owned())),
                    )),
                    Box::new(Term::Var("n".to_owned())),
                )),
            )),
        );
        assert_eq!(result, expected)
    }
}

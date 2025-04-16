use super::terms::{Term, Var};
use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    language::untyped::Untyped,
    terms::{App, Lambda, Variable},
    Parse,
};
use std::collections::VecDeque;

pub mod lexer;

pub fn to_err(knd: ErrorKind) -> Error {
    Error {
        kind: knd,
        loc: ErrorLocation::Parse,
    }
}

use lexer::{lex, Token};

impl Parse for Term {
    fn parse(mut input: String) -> Result<Self, Error> {
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
            None => {
                return Err(to_err(ErrorKind::MissingInput(
                    "Closing Parenthesis".to_string(),
                )))
            }
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
        None => Err(to_err(ErrorKind::MissingInput(format!("Token {expected}")))),
        Some(tok) => {
            if tok == *expected {
                Ok(tok)
            } else {
                Err(to_err(ErrorKind::UnexpectedRule {
                    found: format!("Token {tok}"),
                    expected: expected.to_string(),
                }))
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
                    Err(to_err(ErrorKind::MissingInput("Variable".to_owned())))
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
        Err(to_err(ErrorKind::MissingInput("Variable".to_owned())))
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

    let lambda_term = Lambda::new(&var, Untyped, body).into();
    if tokens.is_empty() {
        Ok(lambda_term)
    } else {
        let next_t = parse_tokens(tokens)?;
        Ok(App::new(lambda_term, next_t).into())
    }
}

fn parse_tokens(tokens: &mut VecDeque<Token>) -> Result<Term, Error> {
    match tokens
        .pop_front()
        .ok_or(to_err(ErrorKind::MissingInput("Term".to_owned())))?
    {
        Token::Lambda => parse_lambda(tokens),
        Token::Char(c) => {
            tokens.insert(0, Token::Char(c));
            let var = parse_var(tokens)?;
            skip_spaces(tokens);
            if tokens.is_empty() {
                Ok(Variable::new(&var).into())
            } else {
                let rest_term = parse_tokens(tokens)?;
                Ok(App::new(Variable::new(&var), rest_term).into())
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
                Ok(App::new(inner_term, remaining_term).into())
            }
        }
        Token::ParensC => Err(to_err(ErrorKind::UnexpectedRule {
            found: "Token )".to_owned(),
            expected: "Term".to_owned(),
        })),
        Token::Dot => Err(to_err(ErrorKind::UnexpectedRule {
            found: "Token .".to_owned(),
            expected: "Term".to_owned(),
        })),
        Token::Space => Err(to_err(ErrorKind::UnexpectedRule {
            found: "Token Space".to_owned(),
            expected: "Term".to_owned(),
        })),
    }
}

pub fn parse(source: &mut String) -> Result<Term, Error> {
    let mut tokens = lex(source);
    parse_tokens(&mut tokens)
}

#[cfg(test)]
mod parser_tests {
    use super::parse;
    use common::{
        language::untyped::Untyped,
        terms::{App, Lambda, Variable},
    };

    #[test]
    fn parse_id() {
        let result = parse(&mut "\\x.x".to_owned()).unwrap();
        let expected = Lambda::new("x", Untyped, Variable::new("x")).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_true() {
        let result = parse(&mut "\\t.\\f.t".to_owned()).unwrap();
        let expected =
            Lambda::new("t", Untyped, Lambda::new("f", Untyped, Variable::new("t"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_false() {
        let result = parse(&mut "\\t.\\f.f".to_owned()).unwrap();
        let expected =
            Lambda::new("t", Untyped, Lambda::new("f", Untyped, Variable::new("f"))).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_if() {
        let result = parse(&mut "\\l.\\m.(l m) n".to_owned()).unwrap();
        let expected = Lambda::new(
            "l",
            Untyped,
            Lambda::new(
                "m",
                Untyped,
                App::new(
                    App::new(Variable::new("l"), Variable::new("m")),
                    Variable::new("n"),
                ),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }
}

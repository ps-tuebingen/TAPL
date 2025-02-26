pub mod errors;
pub mod lexer;

use super::Term;
use errors::Error;
use lexer::{lex, Token};
use std::collections::VecDeque;

pub fn parse(source: String) -> Result<Term, Error> {
    let mut source = source.trim().to_owned();
    let mut tokens = VecDeque::from(lex(&mut source)?);
    let t = parse_term(&mut tokens)?;
    if tokens.is_empty() {
        Ok(t)
    } else {
        Err(Error::RemainingInput(tokens.into()))
    }
}

fn consume_token(tokens: &mut VecDeque<Token>, token: Token) -> Result<(), Error> {
    if let Some(tok) = tokens.pop_front() {
        if tok == token {
            Ok(())
        } else {
            Err(Error::UnexpectedToken(tok))
        }
    } else {
        Err(Error::UnexpectedEOI)
    }
}

fn parse_term(tokens: &mut VecDeque<Token>) -> Result<Term, Error> {
    let fst = if let Some(tok) = tokens.pop_front() {
        tok
    } else {
        return Err(Error::UnexpectedEOI);
    };

    match fst {
        Token::True => Ok(Term::True),
        Token::False => Ok(Term::False),
        Token::Zero => Ok(Term::Zero),
        Token::Succ => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(Term::Succ(Box::new(inner)))
        }
        Token::Pred => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(Term::Pred(Box::new(inner)))
        }
        Token::If => {
            let ifc = parse_term(tokens)?;
            consume_token(tokens, Token::Then)?;
            let thenc = parse_term(tokens)?;
            consume_token(tokens, Token::Else)?;
            let elsec = parse_term(tokens)?;
            Ok(Term::If(Box::new(ifc), Box::new(thenc), Box::new(elsec)))
        }
        Token::IsZero => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(Term::IsZero(Box::new(inner)))
        }
        Token::ParenO => {
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(inner)
        }
        Token::Digit(d) => {
            let mut digits = vec![d];
            let mut next = tokens.pop_front();
            while let Some(Token::Digit(d)) = next {
                digits.push(d);
                next = tokens.pop_front();
            }
            if let Some(t) = next {
                tokens.push_front(t);
            }
            Ok(digits_to_term(digits))
        }
        _ => Err(Error::UnexpectedToken(fst)),
    }
}

fn digits_to_term(digits: Vec<u8>) -> Term {
    let mut num = digits_to_num(digits);
    let mut term = Term::Zero;
    while num > 0 {
        term = Term::Succ(Box::new(term));
        num -= 1;
    }
    term
}

fn digits_to_num(mut digits: Vec<u8>) -> u64 {
    let mut num = 0;
    while !digits.is_empty() {
        let next_pow = digits.len() - 1;
        let next = digits.remove(0);
        num += next as u64 * 10_u64.pow(next_pow as u32);
    }
    num
}

#[cfg(test)]
mod parser_tests {
    use super::{digits_to_num, digits_to_term, parse, Term};

    #[test]
    fn parse_true() {
        let result = parse("True".to_owned()).unwrap();
        let expected = Term::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_false() {
        let result = parse("False".to_owned()).unwrap();
        let expected = Term::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_if() {
        let result = parse("If True Then False Else True".to_owned()).unwrap();
        let expected = Term::If(
            Box::new(Term::True),
            Box::new(Term::False),
            Box::new(Term::True),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_zero() {
        let result = parse("Zero".to_owned()).unwrap();
        let expected = Term::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_succ() {
        let result = parse("Succ(True)".to_owned()).unwrap();
        let expected = Term::Succ(Box::new(Term::True));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_pred() {
        let result = parse("Pred(Zero)".to_owned()).unwrap();
        let expected = Term::Pred(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn prase_iszero() {
        let result = parse("IsZero(Zero)".to_owned()).unwrap();
        let expected = Term::IsZero(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_parens() {
        let result = parse("(True)".to_owned()).unwrap();
        let expected = Term::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn dig2num() {
        let result = digits_to_num(vec![1, 2, 3]);
        let expected = 123;
        assert_eq!(result, expected)
    }

    #[test]
    fn dig2term() {
        let result = digits_to_term(vec![1, 2]);
        let expected = Term::Succ(Box::new(Term::Succ(Box::new(Term::Succ(Box::new(
            Term::Succ(Box::new(Term::Succ(Box::new(Term::Succ(Box::new(
                Term::Succ(Box::new(Term::Succ(Box::new(Term::Succ(Box::new(
                    Term::Succ(Box::new(Term::Succ(Box::new(Term::Succ(Box::new(
                        Term::Zero,
                    )))))),
                )))))),
            )))))),
        ))))));
        assert_eq!(result, expected)
    }
}

pub mod errors;
pub mod lexer;

use super::Term;
use errors::Error;
use lexer::{lex, Token};
use std::collections::VecDeque;

pub fn parse(source: &mut String) -> Result<Term, Error> {
    let mut tokens = VecDeque::from(lex(source)?);
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
        _ => Err(Error::UnexpectedToken(fst)),
    }
}

#[cfg(test)]
mod parser_tests {
    use super::{parse, Term};

    #[test]
    fn parse_true() {
        let result = parse(&mut "True".to_owned()).unwrap();
        let expected = Term::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_false() {
        let result = parse(&mut "False".to_owned()).unwrap();
        let expected = Term::False;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_if() {
        let result = parse(&mut "If True Then False Else True".to_owned()).unwrap();
        let expected = Term::If(
            Box::new(Term::True),
            Box::new(Term::False),
            Box::new(Term::True),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_zero() {
        let result = parse(&mut "Zero".to_owned()).unwrap();
        let expected = Term::Zero;
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_succ() {
        let result = parse(&mut "Succ(True)".to_owned()).unwrap();
        let expected = Term::Succ(Box::new(Term::True));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_pred() {
        let result = parse(&mut "Pred(Zero)".to_owned()).unwrap();
        let expected = Term::Pred(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn prase_iszero() {
        let result = parse(&mut "IsZero(Zero)".to_owned()).unwrap();
        let expected = Term::IsZero(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_parens() {
        let result = parse(&mut "(True)".to_owned()).unwrap();
        let expected = Term::True;
        assert_eq!(result, expected)
    }
}

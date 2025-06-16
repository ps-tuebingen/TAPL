use super::errors::Error;
use parse::{
    errors::{MissingInput, RemainingInput, UnexpectedRule},
    Parse, Rule,
};
use pest::iterators::Pair;
pub mod lexer;

use super::terms::Term;
use lexer::{lex, Token};
use std::collections::VecDeque;
use syntax::terms::{False, If, IsZero, Num, Pred, Succ, True};

impl Parse for Term {
    type ParseError = Error;

    fn rule() -> Rule {
        Rule::term
    }

    fn from_pair(p: Pair<'_, Rule>) -> Result<Self, Self::ParseError> {
        todo!()
    }
}

pub fn parse(source: String) -> Result<Term, Error> {
    let mut source = source.trim().to_owned();
    let mut tokens = VecDeque::from(lex(&mut source)?);
    let t = parse_term(&mut tokens)?;
    if tokens.is_empty() {
        Ok(t)
    } else {
        Err(RemainingInput::new(
            &tokens
                .iter()
                .map(|tok| tok.to_string())
                .collect::<Vec<String>>()
                .join(","),
        )
        .into())
    }
}

fn consume_token(tokens: &mut VecDeque<Token>, token: Token) -> Result<(), Error> {
    if let Some(tok) = tokens.pop_front() {
        if tok == token {
            Ok(())
        } else {
            Err(UnexpectedRule::new(Rule::WHITESPACE, &format!("{token}")).into())
        }
    } else {
        Err(MissingInput::new(&token.to_string()).into())
    }
}

fn parse_term(tokens: &mut VecDeque<Token>) -> Result<Term, Error> {
    let fst = if let Some(tok) = tokens.pop_front() {
        tok
    } else {
        return Err(MissingInput::new("Term").into());
    };

    match fst {
        Token::True => Ok(True::new().into()),
        Token::False => Ok(False::new().into()),
        Token::Zero => Ok(Num::new(0).into()),
        Token::Succ => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(Succ::new(inner).into())
        }
        Token::Pred => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(Pred::new(inner).into())
        }
        Token::If => {
            let ifc = parse_term(tokens)?;
            consume_token(tokens, Token::BrackO)?;
            let thenc = parse_term(tokens)?;
            consume_token(tokens, Token::BrackC)?;
            consume_token(tokens, Token::Else)?;
            consume_token(tokens, Token::BrackO)?;
            let elsec = parse_term(tokens)?;
            consume_token(tokens, Token::BrackC)?;
            Ok(If::new(ifc, thenc, elsec).into())
        }
        Token::IsZero => {
            consume_token(tokens, Token::ParenO)?;
            let inner = parse_term(tokens)?;
            consume_token(tokens, Token::ParenC)?;
            Ok(IsZero::new(inner).into())
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
        _ => Err(UnexpectedRule::new(Rule::WHITESPACE, "Term").into()),
    }
}

fn digits_to_term(digits: Vec<u8>) -> Term {
    let mut num = digits_to_num(digits);
    let mut term = Num::new(0).into();
    while num > 0 {
        term = Succ::new(term).into();
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
    use super::{digits_to_num, digits_to_term, parse};
    use syntax::terms::{False, If, IsZero, Num, Pred, Succ, True};

    #[test]
    fn parse_true() {
        let result = parse("True".to_owned()).unwrap();
        let expected = True::new().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_false() {
        let result = parse("False".to_owned()).unwrap();
        let expected = False::new().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_if() {
        let result = parse("If (true) { false } else { true }".to_owned()).unwrap();
        let expected = If::new(True::new(), False::new(), True::new()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_zero() {
        let result = parse("Zero".to_owned()).unwrap();
        let expected = Num::new(0).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_succ() {
        let result = parse("Succ(True)".to_owned()).unwrap();
        let expected = Succ::new(True::new()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_pred() {
        let result = parse("Pred(Zero)".to_owned()).unwrap();
        let expected = Pred::new(Num::new(0)).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn prase_iszero() {
        let result = parse("IsZero(Zero)".to_owned()).unwrap();
        let expected = IsZero::new(Num::new(0)).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_parens() {
        let result = parse("(True)".to_owned()).unwrap();
        let expected = True::new().into();
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
        let expected = Succ::new(Succ::new(Succ::new(Succ::new(Succ::new(Succ::new(
            Succ::new(Succ::new(Succ::new(Succ::new(Succ::new(Succ::new(
                Num::new(0),
            )))))),
        ))))))
        .into();
        assert_eq!(result, expected)
    }
}

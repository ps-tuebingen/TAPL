use super::{Error, MissingInput, RemainingInput};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    True,
    False,
    If,
    Else,
    Zero,
    Succ,
    Pred,
    IsZero,
    ParenO,
    ParenC,
    BrackO,
    BrackC,
    Digit(u8),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::True => f.write_str("True"),
            Token::False => f.write_str("Fase"),
            Token::If => f.write_str("If"),
            Token::Else => f.write_str("Else"),
            Token::Zero => f.write_str("Zero"),
            Token::Succ => f.write_str("Succ"),
            Token::Pred => f.write_str("Pred"),
            Token::IsZero => f.write_str("IsZero"),
            Token::ParenO => f.write_str("("),
            Token::ParenC => f.write_str(")"),
            Token::BrackO => f.write_str("{"),
            Token::BrackC => f.write_str("}"),
            Token::Digit(dig) => write!(f, "{dig}"),
        }
    }
}

fn remove_n(source: &mut String, n: usize) -> Result<(), Error> {
    if source.len() < n {
        Err(MissingInput::new(&format!("{n} Tokens")).into())
    } else {
        for _ in 0..n {
            source.remove(0);
        }
        Ok(())
    }
}

fn remove_whitespace(source: &mut String) {
    if source.starts_with(' ') {
        source.remove(0);
        remove_whitespace(source);
    }
}

pub fn lex(source: &mut String) -> Result<Vec<Token>, Error> {
    if source.is_empty() {
        return Err(MissingInput::new("Program").into());
    }
    *source = source.to_lowercase();
    let mut tokens = vec![];
    while !source.is_empty() {
        let c1 = source.remove(0);
        match c1.to_ascii_uppercase() {
            'T' => {
                if source.starts_with("rue") {
                    remove_n(source, 3)?;
                    tokens.push(Token::True);
                } else {
                    return Err(MissingInput::new("rue").into());
                }
            }
            'F' => {
                if source.starts_with("alse") {
                    remove_n(source, 4)?;
                    tokens.push(Token::False);
                } else {
                    return Err(MissingInput::new("alse").into());
                }
            }
            'Z' => {
                if source.starts_with("ero") {
                    remove_n(source, 3)?;
                    tokens.push(Token::Zero);
                } else {
                    return Err(MissingInput::new("ero").into());
                }
            }
            'S' => {
                if source.starts_with("ucc") {
                    remove_n(source, 3)?;
                    tokens.push(Token::Succ);
                } else {
                    return Err(MissingInput::new("ucc").into());
                }
            }
            'P' => {
                if source.starts_with("red") {
                    remove_n(source, 3)?;
                    tokens.push(Token::Pred);
                } else {
                    return Err(MissingInput::new("red").into());
                }
            }
            'I' => {
                if source.starts_with("f") {
                    remove_n(source, 1)?;
                    tokens.push(Token::If);
                } else if source.starts_with("szero") {
                    remove_n(source, 5)?;
                    tokens.push(Token::IsZero);
                } else {
                    return Err(MissingInput::new("f or szero").into());
                }
            }
            'E' => {
                if source.starts_with("lse") {
                    remove_n(source, 3)?;
                    tokens.push(Token::Else);
                } else {
                    return Err(MissingInput::new("lse").into());
                }
            }
            '(' => tokens.push(Token::ParenO),
            ')' => tokens.push(Token::ParenC),
            '{' => tokens.push(Token::BrackO),
            '}' => tokens.push(Token::BrackC),
            c if c.is_numeric() => {
                let dig = c.to_string().parse::<u8>().unwrap();
                tokens.push(Token::Digit(dig));
            }
            _ => return Err(RemainingInput::new(&c1.to_string()).into()),
        }
        remove_whitespace(source)
    }
    if tokens.is_empty() {
        Err(MissingInput::new("Term").into())
    } else {
        Ok(tokens)
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::{lex, Token};

    #[test]
    fn lex_true() {
        let result = lex(&mut "True".to_owned()).unwrap();
        let expected = vec![Token::True];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_false() {
        let result = lex(&mut "False".to_owned()).unwrap();
        let expected = vec![Token::False];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_if() {
        let result = lex(&mut "If True False Else True".to_owned()).unwrap();
        let expected = vec![
            Token::If,
            Token::True,
            Token::False,
            Token::Else,
            Token::True,
        ];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_zero() {
        let result = lex(&mut "Zero".to_owned()).unwrap();
        let expected = vec![Token::Zero];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_succ() {
        let result = lex(&mut "Succ(True)".to_owned()).unwrap();
        let expected = vec![Token::Succ, Token::ParenO, Token::True, Token::ParenC];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_pred() {
        let result = lex(&mut "Pred(Zero)".to_owned()).unwrap();
        let expected = vec![Token::Pred, Token::ParenO, Token::Zero, Token::ParenC];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_iszero() {
        let result = lex(&mut "IsZero(Zero)".to_owned()).unwrap();
        let expected = vec![Token::IsZero, Token::ParenO, Token::Zero, Token::ParenC];
        assert_eq!(result, expected)
    }

    #[test]
    fn lex_num() {
        let result = lex(&mut "123".to_owned()).unwrap();
        let expected = vec![Token::Digit(1), Token::Digit(2), Token::Digit(3)];
        assert_eq!(result, expected)
    }
}

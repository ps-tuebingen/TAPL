use std::fmt;

pub trait Parse: Sized {
    type Rule;
    type ParseError: std::error::Error + From<pest::error::Error<Self::Rule>>;
    fn parse(sourcte: String) -> Result<Self, Self::ParseError>;
}

#[derive(Debug)]
pub struct MissingInput {
    input: String,
}

impl MissingInput {
    pub fn new(input: &str) -> MissingInput {
        MissingInput {
            input: input.to_owned(),
        }
    }
}

impl fmt::Display for MissingInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing Input {}", self.input)
    }
}

impl std::error::Error for MissingInput {}

#[derive(Debug)]
pub struct RemainingInput {
    remaining: String,
}

impl RemainingInput {
    pub fn new(remaining: &str) -> RemainingInput {
        RemainingInput {
            remaining: remaining.to_owned(),
        }
    }
}

impl fmt::Display for RemainingInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Remaining Input {}", self.remaining)
    }
}

impl std::error::Error for RemainingInput {}

#[derive(Debug)]
pub struct UnexpectedRule<R>
where
    R: fmt::Debug,
{
    found: R,
    expected: String,
}

impl<R> UnexpectedRule<R>
where
    R: fmt::Debug,
{
    pub fn new(found: R, expected: &str) -> UnexpectedRule<R> {
        UnexpectedRule {
            found,
            expected: expected.to_owned(),
        }
    }
}

impl<R> fmt::Display for UnexpectedRule<R>
where
    R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Unexpected rule {:?}, expected: {}",
            self.found, self.expected
        )
    }
}

impl<R> std::error::Error for UnexpectedRule<R> where R: fmt::Debug {}

#[derive(Debug)]
pub struct UnknownKeyword {
    kw: String,
}

impl UnknownKeyword {
    pub fn new(kw: &str) -> UnknownKeyword {
        UnknownKeyword { kw: kw.to_owned() }
    }
}

impl fmt::Display for UnknownKeyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown keyword {}", self.kw)
    }
}

impl std::error::Error for UnknownKeyword {}

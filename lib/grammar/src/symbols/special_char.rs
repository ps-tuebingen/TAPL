use super::Symbol;
use std::fmt;

/// Special characters used in [`super::Symbol`]
/// Can be printed in textual representation or in latex representation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecialChar {
    Number,
    Lambda,
    Mu,
    Gamma,
    Forall,
    Exists,
    Top,
    Bot,
    Dot,
    Colon,
    DoubleColon,
    Exclamation,
    LessColon,
    Comma,
    Equals,
    Plus,
    Times,
    Empty,
    Arrow,
    DoubleArrow,
    Mapsto,
    Space,
    ColonEq,
    BrackO,
    BrackC,
    ParenO,
    ParenC,
    SqBrackO,
    SqBrackC,
    AngBrackO,
    AngBrackC,
    Star,
    LessEq,
    Ellipses,
    Pipe,
    Element,
}

impl From<SpecialChar> for Symbol {
    fn from(sc: SpecialChar) -> Self {
        Self::SpecialChar(sc)
    }
}

impl fmt::Display for SpecialChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number => f.write_str("int"),
            Self::Lambda => f.write_str("\\"),
            Self::Mu => f.write_str("mu"),
            Self::Gamma => f.write_str("Gamma"),
            Self::Forall => f.write_str("forall"),
            Self::Exists => f.write_str("exists"),
            Self::Top => f.write_str("Top"),
            Self::Bot => f.write_str("Bot"),
            Self::Dot => f.write_str("."),
            Self::Colon => f.write_str(":"),
            Self::DoubleColon => f.write_str("::"),
            Self::Exclamation => f.write_str("!"),
            Self::LessColon => f.write_str("<:"),
            Self::Comma => f.write_str(","),
            Self::Equals => f.write_str("="),
            Self::Plus => f.write_str("+"),
            Self::Times => f.write_str("x"),
            Self::Empty => f.write_str(""),
            Self::Arrow => f.write_str("->"),
            Self::DoubleArrow => f.write_str("=>"),
            Self::Mapsto => f.write_str("-->"),
            Self::Space => f.write_str(" "),
            Self::ColonEq => f.write_str(":="),
            Self::BrackO => f.write_str("{"),
            Self::BrackC => f.write_str("}"),
            Self::ParenO => f.write_str("("),
            Self::ParenC => f.write_str(")"),
            Self::SqBrackO => f.write_str("["),
            Self::SqBrackC => f.write_str("]"),
            Self::AngBrackO => f.write_str("<"),
            Self::AngBrackC => f.write_str(">"),
            Self::Star => f.write_str("*"),
            Self::LessEq => f.write_str("<="),
            Self::Ellipses => f.write_str("..."),
            Self::Pipe => f.write_str("|"),
            Self::Element => f.write_str("in"),
        }
    }
}

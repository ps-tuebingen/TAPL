use super::Symbol;
use std::fmt;

#[derive(Clone, Debug)]
pub enum SpecialChar {
    Number,
    Lambda,
    Mu,
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
}

impl From<SpecialChar> for Symbol {
    fn from(sc: SpecialChar) -> Symbol {
        Symbol::SpecialChar(sc)
    }
}

impl fmt::Display for SpecialChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialChar::Number => f.write_str("int"),
            SpecialChar::Lambda => f.write_str("\\"),
            SpecialChar::Mu => f.write_str("mu"),
            SpecialChar::Forall => f.write_str("forall"),
            SpecialChar::Exists => f.write_str("exists"),
            SpecialChar::Top => f.write_str("Top"),
            SpecialChar::Bot => f.write_str("Bot"),
            SpecialChar::Dot => f.write_str("."),
            SpecialChar::Colon => f.write_str(":"),
            SpecialChar::DoubleColon => f.write_str("::"),
            SpecialChar::Exclamation => f.write_str("!"),
            SpecialChar::LessColon => f.write_str("<:"),
            SpecialChar::Comma => f.write_str(","),
            SpecialChar::Equals => f.write_str("="),
            SpecialChar::Plus => f.write_str("+"),
            SpecialChar::Times => f.write_str("*"),
            SpecialChar::Empty => f.write_str(""),
            SpecialChar::Arrow => f.write_str("->"),
            SpecialChar::DoubleArrow => f.write_str("=>"),
            SpecialChar::Space => f.write_str(" "),
            SpecialChar::ColonEq => f.write_str(":="),
            SpecialChar::BrackO => f.write_str("{"),
            SpecialChar::BrackC => f.write_str("}"),
            SpecialChar::ParenO => f.write_str("("),
            SpecialChar::ParenC => f.write_str(")"),
            SpecialChar::SqBrackO => f.write_str("["),
            SpecialChar::SqBrackC => f.write_str("]"),
            SpecialChar::AngBrackO => f.write_str("<"),
            SpecialChar::AngBrackC => f.write_str(">"),
            SpecialChar::Star => f.write_str("*"),
        }
    }
}

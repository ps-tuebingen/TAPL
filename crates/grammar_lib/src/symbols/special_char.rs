use super::Symbol;

#[derive(Clone)]
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

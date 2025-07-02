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
}

impl From<SpecialChar> for Symbol {
    fn from(sc: SpecialChar) -> Symbol {
        Symbol::SpecialChar(sc)
    }
}

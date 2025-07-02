use super::Symbol;

#[derive(Clone)]
pub enum Keyword {
    Let,
    If,
    Try,
    Nil,
    Cons,
    Head,
    Tail,
    IsNil,
    Succ,
    Pred,
    IsZero,
    Something,
    Left,
    Right,
    Nothing,
    Raise,
    Err,
    Fold,
    Unfold,
    Ref,
    Fix,

    Source,
    Sink,
    Reference,
    Optional,
    List,
}

impl From<Keyword> for Symbol {
    fn from(kw: Keyword) -> Symbol {
        Symbol::Keyword(kw)
    }
}

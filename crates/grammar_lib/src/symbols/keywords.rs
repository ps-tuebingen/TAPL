use super::Symbol;

#[derive(Clone)]
pub enum Keyword {
    As,
    In,

    Let,
    If,
    Else,
    Try,
    Catch,
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
    Unit,
    True,
    False,
    Fst,
    Snd,

    Source,
    Sink,
    Reference,
    Optional,
    List,
    UnitTy,
    Nat,
    Bool,
}

impl From<Keyword> for Symbol {
    fn from(kw: Keyword) -> Symbol {
        Symbol::Keyword(kw)
    }
}

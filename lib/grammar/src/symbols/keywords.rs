use super::Symbol;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::As => f.write_str("as"),
            Keyword::In => f.write_str("in"),
            Keyword::Let => f.write_str("let"),
            Keyword::If => f.write_str("if"),
            Keyword::Else => f.write_str("else"),
            Keyword::Try => f.write_str("try"),
            Keyword::Catch => f.write_str("catch"),
            Keyword::Nil => f.write_str("nil"),
            Keyword::Cons => f.write_str("cons"),
            Keyword::Head => f.write_str("head"),
            Keyword::Tail => f.write_str("tail"),
            Keyword::IsNil => f.write_str("isnil"),
            Keyword::Succ => f.write_str("succ"),
            Keyword::Pred => f.write_str("pred"),
            Keyword::IsZero => f.write_str("iszero"),
            Keyword::Something => f.write_str("something"),
            Keyword::Left => f.write_str("left"),
            Keyword::Right => f.write_str("right"),
            Keyword::Nothing => f.write_str("nothing"),
            Keyword::Raise => f.write_str("raise"),
            Keyword::Err => f.write_str("error"),
            Keyword::Fold => f.write_str("fold"),
            Keyword::Unfold => f.write_str("unfold"),
            Keyword::Ref => f.write_str("ref"),
            Keyword::Fix => f.write_str("fix"),
            Keyword::Unit => f.write_str("unit"),
            Keyword::True => f.write_str("true"),
            Keyword::False => f.write_str("false"),
            Keyword::Fst => f.write_str("fst"),
            Keyword::Snd => f.write_str("snd"),
            Keyword::Source => f.write_str("Source"),
            Keyword::Sink => f.write_str("Sink"),
            Keyword::Reference => f.write_str("Ref"),
            Keyword::Optional => f.write_str("Optional"),
            Keyword::List => f.write_str("List"),
            Keyword::UnitTy => f.write_str("Unit"),
            Keyword::Nat => f.write_str("Nat"),
            Keyword::Bool => f.write_str("Bool"),
        }
    }
}

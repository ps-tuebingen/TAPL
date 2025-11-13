use super::Symbol;
use std::fmt;

/// Keyword used in [`super::Symbol`]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
    As,
    In,
    Let,
    If,
    Else,
    Try,
    With,
    Catch,
    Case,
    Of,
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
    fn from(kw: Keyword) -> Self {
        Self::Keyword(kw)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::As => f.write_str("as"),
            Self::In => f.write_str("in"),
            Self::Let => f.write_str("let"),
            Self::If => f.write_str("if"),
            Self::Else => f.write_str("else"),
            Self::Try => f.write_str("try"),
            Self::With => f.write_str("with"),
            Self::Catch => f.write_str("catch"),
            Self::Of => f.write_str("of"),
            Self::Case => f.write_str("case"),
            Self::Nil => f.write_str("nil"),
            Self::Cons => f.write_str("cons"),
            Self::Head => f.write_str("head"),
            Self::Tail => f.write_str("tail"),
            Self::IsNil => f.write_str("isnil"),
            Self::Succ => f.write_str("succ"),
            Self::Pred => f.write_str("pred"),
            Self::IsZero => f.write_str("iszero"),
            Self::Something => f.write_str("something"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Nothing => f.write_str("nothing"),
            Self::Raise => f.write_str("raise"),
            Self::Err => f.write_str("error"),
            Self::Fold => f.write_str("fold"),
            Self::Unfold => f.write_str("unfold"),
            Self::Ref => f.write_str("ref"),
            Self::Fix => f.write_str("fix"),
            Self::Unit => f.write_str("unit"),
            Self::True => f.write_str("true"),
            Self::False => f.write_str("false"),
            Self::Fst => f.write_str("fst"),
            Self::Snd => f.write_str("snd"),
            Self::Source => f.write_str("Source"),
            Self::Sink => f.write_str("Sink"),
            Self::Reference => f.write_str("Ref"),
            Self::Optional => f.write_str("Optional"),
            Self::List => f.write_str("List"),
            Self::UnitTy => f.write_str("Unit"),
            Self::Nat => f.write_str("Nat"),
            Self::Bool => f.write_str("Bool"),
        }
    }
}

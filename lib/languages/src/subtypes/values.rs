use super::{Subtypes, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, ValueGroup,
    Variant,
};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    FromVariants,
    LatexFmt,
    LangDisplay,
    Debug,
    PartialEq,
    Eq,
    Clone,
)]
#[Lang(Subtypes)]
pub enum Value {
    Lambda(Lambda<Subtypes>),
    Unit(Unit<Subtypes>),
    Record(Record<Subtypes>),
    Variant(Variant<Subtypes>),
    Nil(Nil<Subtypes>),
    Cons(Cons<Subtypes>),
    Loc(Loc<Subtypes>),
    Num(Num<Subtypes>),
    True(True<Subtypes>),
    False(False<Subtypes>),
}

impl ValueTrait for Value {
    type Lang = Subtypes;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<Subtypes>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Subtypes>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Subtypes>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }

    fn into_nil(self) -> Option<Nil<Subtypes>> {
        if let Self::Nil(nil) = self {
            Some(nil)
        } else {
            None
        }
    }

    fn into_cons(self) -> Option<Cons<Subtypes>> {
        if let Self::Cons(cons) = self {
            Some(cons)
        } else {
            None
        }
    }

    fn into_loc(self) -> Option<Loc<Subtypes>> {
        if let Self::Loc(loc) = self {
            Some(loc)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<Subtypes>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_true(self) -> Option<True<Subtypes>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<Subtypes>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }
}

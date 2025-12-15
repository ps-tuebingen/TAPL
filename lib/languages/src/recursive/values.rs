use super::{Recursive, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    False, Fold, Lambda, Num, Pair, Record, True, Unit, Value as ValueTrait, ValueGroup, Variant,
};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    FromVariants,
    LatexFmt,
    LangDisplay,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(Recursive)]
pub enum Value {
    True(True<Recursive>),
    False(False<Recursive>),
    Unit(Unit<Recursive>),
    Num(Num<Recursive>),
    Lambda(Lambda<Recursive>),
    Fold(Fold<Recursive>),
    Pair(Pair<Recursive>),
    Record(Record<Recursive>),
    Variant(Variant<Recursive>),
}

impl ValueTrait for Value {
    type Lang = Recursive;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_true(self) -> Option<True<Recursive>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<Recursive>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<Recursive>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_lambda(self) -> Option<Lambda<Recursive>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_fold(self) -> Option<Fold<Recursive>> {
        if let Self::Fold(fld) = self {
            Some(fld)
        } else {
            None
        }
    }

    fn into_pair(self) -> Option<Pair<Recursive>> {
        if let Self::Pair(pair) = self {
            Some(pair)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Recursive>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Recursive>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }
}

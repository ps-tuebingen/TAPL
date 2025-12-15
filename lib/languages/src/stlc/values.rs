use super::{Stlc, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    Cons, False, Lambda, Left, Nil, Nothing, Num, Pair, Record, Right, Something, True, Tuple,
    Unit, Value as ValueTrait, ValueGroup, Variant,
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
#[Lang(Stlc)]
pub enum Value {
    Lambda(Lambda<Stlc>),
    Unit(Unit<Stlc>),
    True(True<Stlc>),
    False(False<Stlc>),
    Num(Num<Stlc>),
    Pair(Pair<Stlc>),
    Tuple(Tuple<Stlc>),
    Record(Record<Stlc>),
    Left(Left<Stlc>),
    Right(Right<Stlc>),
    Variant(Variant<Stlc>),
    Nothing(Nothing<Stlc>),
    Something(Something<Stlc>),
    Nil(Nil<Stlc>),
    Cons(Cons<Stlc>),
}

impl ValueTrait for Value {
    type Lang = Stlc;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<Stlc>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_true(self) -> Option<True<Stlc>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<Stlc>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<Stlc>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_pair(self) -> Option<Pair<Stlc>> {
        if let Self::Pair(pair) = self {
            Some(pair)
        } else {
            None
        }
    }

    fn into_tuple(self) -> Option<Tuple<Stlc>> {
        if let Self::Tuple(tup) = self {
            Some(tup)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Stlc>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_left(self) -> Option<Left<Stlc>> {
        if let Self::Left(lft) = self {
            Some(lft)
        } else {
            None
        }
    }

    fn into_right(self) -> Option<Right<Stlc>> {
        if let Self::Right(right) = self {
            Some(right)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Stlc>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }

    fn into_nothing(self) -> Option<Nothing<Stlc>> {
        if let Self::Nothing(not) = self {
            Some(not)
        } else {
            None
        }
    }

    fn into_something(self) -> Option<Something<Stlc>> {
        if let Self::Something(somet) = self {
            Some(somet)
        } else {
            None
        }
    }

    fn into_nil(self) -> Option<Nil<Stlc>> {
        if let Self::Nil(nil) = self {
            Some(nil)
        } else {
            None
        }
    }

    fn into_cons(self) -> Option<Cons<Stlc>> {
        if let Self::Cons(cons) = self {
            Some(cons)
        } else {
            None
        }
    }
}

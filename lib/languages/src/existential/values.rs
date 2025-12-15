use super::{Existential, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    False, Lambda, Num, Pack, Record, True, Unit, Value as ValueTrait, ValueGroup,
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
#[Lang(Existential)]
pub enum Value {
    Unit(Unit<Existential>),
    Lambda(Lambda<Existential>),
    Pack(Pack<Existential>),
    Num(Num<Existential>),
    Record(Record<Existential>),
    True(True<Existential>),
    False(False<Existential>),
}

impl ValueTrait for Value {
    type Lang = Existential;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<Existential>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_pack(self) -> Option<Pack<Existential>> {
        if let Self::Pack(pack) = self {
            Some(pack)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<Existential>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Existential>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_true(self) -> Option<True<Existential>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }

    fn into_false(self) -> Option<False<Existential>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }
}

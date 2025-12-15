use super::{FOmega, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{
    False, Lambda, Num, Pack, Record, True, TyLambda, Unit, Value as ValueTrait, ValueGroup,
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
#[Lang(FOmega)]
pub enum Value {
    Lambda(Lambda<FOmega>),
    TyLambda(TyLambda<FOmega>),
    Pack(Pack<FOmega>),
    Record(Record<FOmega>),
    True(True<FOmega>),
    False(False<FOmega>),
    Unit(Unit<FOmega>),
    Num(Num<FOmega>),
}

impl ValueTrait for Value {
    type Lang = FOmega;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<FOmega>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_tylambda(self) -> Option<TyLambda<FOmega>> {
        if let Self::TyLambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_pack(self) -> Option<Pack<FOmega>> {
        if let Self::Pack(pack) = self {
            Some(pack)
        } else {
            None
        }
    }
    fn into_record(self) -> Option<Record<FOmega>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_true(self) -> Option<True<FOmega>> {
        if let Self::True(tru) = self {
            Some(tru)
        } else {
            None
        }
    }
    fn into_false(self) -> Option<False<FOmega>> {
        if let Self::False(fls) = self {
            Some(fls)
        } else {
            None
        }
    }
    fn into_num(self) -> Option<Num<FOmega>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }
}

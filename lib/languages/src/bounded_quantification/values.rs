use super::{BoundedQuantification, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

#[derive(
    Spanned,
    IntoTerm,
    GrammarDescribe,
    LatexFmt,
    FromVariants,
    LangDisplay,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(BoundedQuantification)]
pub enum Value {
    Lambda(Lambda<BoundedQuantification>),
    LambdaSub(LambdaSub<BoundedQuantification>),
    Pack(Pack<BoundedQuantification>),
    Num(Num<BoundedQuantification>),
    Record(Record<BoundedQuantification>),
}

impl ValueTrait for Value {
    type Lang = BoundedQuantification;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<BoundedQuantification>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_lambdasub(self) -> Option<LambdaSub<BoundedQuantification>> {
        if let Self::LambdaSub(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_pack(self) -> Option<Pack<BoundedQuantification>> {
        if let Self::Pack(pack) = self {
            Some(pack)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<BoundedQuantification>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<BoundedQuantification>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }
}

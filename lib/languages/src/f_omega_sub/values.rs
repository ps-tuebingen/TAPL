use super::{FOmegaSub, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt, Spanned};
use syntax::values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait, ValueGroup};

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
#[Lang(FOmegaSub)]
pub enum Value {
    Lambda(Lambda<FOmegaSub>),
    LambdaSub(LambdaSub<FOmegaSub>),
    Pack(Pack<FOmegaSub>),
    Record(Record<FOmegaSub>),
    Num(Num<FOmegaSub>),
}

impl ValueTrait for Value {
    type Lang = FOmegaSub;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Option<Lambda<FOmegaSub>> {
        if let Self::Lambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_lambdasub(self) -> Option<LambdaSub<FOmegaSub>> {
        if let Self::LambdaSub(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_pack(self) -> Option<Pack<FOmegaSub>> {
        if let Self::Pack(pack) = self {
            Some(pack)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<FOmegaSub>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_num(self) -> Option<Num<FOmegaSub>> {
        if let Self::Num(num) = self {
            Some(num)
        } else {
            None
        }
    }
}

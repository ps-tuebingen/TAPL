use super::BoundedQuantification;
use macros::{
    FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm, Spanned, SubstType,
    Subtypecheck,
};
use syntax::types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeGroup, TypeVariable};

#[derive(
    Spanned,
    GrammarDescribe,
    SubstType,
    LatexFmt,
    FromVariants,
    LangDisplay,
    NoNorm,
    NoKinds,
    Subtypecheck,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
#[Lang(BoundedQuantification)]
pub enum Type {
    Var(TypeVariable<BoundedQuantification>),
    Top(Top<BoundedQuantification>),
    Nat(Nat<BoundedQuantification>),
    Fun(Fun<BoundedQuantification>),
    Forall(ForallBounded<BoundedQuantification>),
    Exists(ExistsBounded<BoundedQuantification>),
    Record(Record<BoundedQuantification>),
}

impl syntax::types::Type for Type {}

impl TypeGroup for Type {
    type Lang = BoundedQuantification;
    fn into_variable(self) -> Option<TypeVariable<BoundedQuantification>> {
        if let Self::Var(var) = self {
            Some(var)
        } else {
            None
        }
    }
    fn into_top(self) -> Option<Top<BoundedQuantification>> {
        if let Self::Top(top) = self {
            Some(top)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<BoundedQuantification>> {
        if let Self::Nat(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<BoundedQuantification>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_forall_bounded(self) -> Option<ForallBounded<BoundedQuantification>> {
        if let Self::Forall(forall) = self {
            Some(forall)
        } else {
            None
        }
    }

    fn into_exists_bounded(self) -> Option<ExistsBounded<BoundedQuantification>> {
        if let Self::Exists(ex) = self {
            Some(ex)
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

use super::FOmegaSub;
use macros::{
    FreeTypeVars, FromVariants, GrammarDescribe, Kindcheck, LangDisplay, LatexFmt, Normalize,
    Spanned, SubstType, Subtypecheck,
};
use syntax::types::{
    ExistsBounded, ForallBounded, Fun, Nat, OpApp, OpLambdaSub, Record, Top, Type as TypeTrait,
    TypeGroup, TypeVariable,
};

#[derive(
    FreeTypeVars,
    Spanned,
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    Normalize,
    Kindcheck,
    Subtypecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(FOmegaSub)]
pub enum Type {
    Var(TypeVariable<FOmegaSub>),
    Top(Top<FOmegaSub>),
    Fun(Fun<FOmegaSub>),
    Forall(ForallBounded<FOmegaSub>),
    OpLambdaSub(OpLambdaSub<FOmegaSub>),
    OpApp(OpApp<FOmegaSub>),
    Exists(ExistsBounded<FOmegaSub>),
    Record(Record<FOmegaSub>),
    Nat(Nat<FOmegaSub>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = FOmegaSub;
    fn into_variable(self) -> Option<TypeVariable<FOmegaSub>> {
        if let Self::Var(var) = self {
            Some(var)
        } else {
            None
        }
    }
    fn into_top(self) -> Option<Top<FOmegaSub>> {
        if let Self::Top(top) = self {
            Some(top)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<FOmegaSub>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_forall_bounded(self) -> Option<ForallBounded<FOmegaSub>> {
        if let Self::Forall(forall) = self {
            Some(forall)
        } else {
            None
        }
    }

    fn into_oplambdasub(self) -> Option<OpLambdaSub<FOmegaSub>> {
        if let Self::OpLambdaSub(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_opapp(self) -> Option<OpApp<FOmegaSub>> {
        if let Self::OpApp(app) = self {
            Some(app)
        } else {
            None
        }
    }

    fn into_exists_bounded(self) -> Option<ExistsBounded<FOmegaSub>> {
        if let Self::Exists(ex) = self {
            Some(ex)
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

    fn into_nat(self) -> Option<Nat<FOmegaSub>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }
}

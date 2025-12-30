use super::FOmega;
use macros::{
    FreeTypeVars, FromVariants, GenerateConstraintsType, GrammarDescribe, Kindcheck, LangDisplay,
    LatexFmt, NoSubtypes, Normalize, SolveConstraint, Spanned, SubstType,
};
use syntax::types::{
    Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, Type as TypeTrait, TypeGroup,
    TypeVariable, Unit,
};

#[derive(
    SolveConstraint,
    GenerateConstraintsType,
    FreeTypeVars,
    Spanned,
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    Normalize,
    Kindcheck,
    NoSubtypes,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(FOmega)]
pub enum Type {
    Var(TypeVariable<FOmega>),
    Fun(Fun<FOmega>),
    Forall(Forall<FOmega>),
    OpLambda(OpLambda<FOmega>),
    OpApp(OpApp<FOmega>),
    Exists(Exists<FOmega>),
    Record(Record<FOmega>),
    Bool(Bool<FOmega>),
    Unit(Unit<FOmega>),
    Nat(Nat<FOmega>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = FOmega;
    fn into_fun(self) -> Option<Fun<FOmega>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_forall(self) -> Option<Forall<FOmega>> {
        if let Self::Forall(forall) = self {
            Some(forall)
        } else {
            None
        }
    }

    fn into_oplambda(self) -> Option<OpLambda<FOmega>> {
        if let Self::OpLambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_opapp(self) -> Option<OpApp<FOmega>> {
        if let Self::OpApp(app) = self {
            Some(app)
        } else {
            None
        }
    }

    fn into_exists(self) -> Option<Exists<FOmega>> {
        if let Self::Exists(ex) = self {
            Some(ex)
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

    fn into_bool(self) -> Option<Bool<FOmega>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_unit(self) -> Option<Unit<FOmega>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<FOmega>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }
}

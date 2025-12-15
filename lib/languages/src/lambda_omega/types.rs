use super::LambdaOmega;
use macros::{
    FromVariants, GrammarDescribe, Kindcheck, LangDisplay, LatexFmt, NoNorm, NoSubtypes, SubstType,
};
use syntax::types::{
    Bool, Forall, Fun, Nat, OpApp, OpLambda, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
};

pub type TypeVar = String;

#[derive(
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    Kindcheck,
    NoSubtypes,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(LambdaOmega)]
pub enum Type {
    Var(TypeVariable<LambdaOmega>),
    Unit(Unit<LambdaOmega>),
    Nat(Nat<LambdaOmega>),
    Bool(Bool<LambdaOmega>),
    OpLambda(OpLambda<LambdaOmega>),
    OpApp(OpApp<LambdaOmega>),
    Fun(Fun<LambdaOmega>),
    Forall(Forall<LambdaOmega>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = LambdaOmega;
    fn into_variable(self) -> Option<TypeVariable<LambdaOmega>> {
        if let Self::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    fn into_unit(self) -> Option<Unit<LambdaOmega>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<LambdaOmega>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<LambdaOmega>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_oplambda(self) -> Option<OpLambda<LambdaOmega>> {
        if let Self::OpLambda(lam) = self {
            Some(lam)
        } else {
            None
        }
    }

    fn into_opapp(self) -> Option<OpApp<LambdaOmega>> {
        if let Self::OpApp(app) = self {
            Some(app)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<LambdaOmega>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_forall(self) -> Option<Forall<LambdaOmega>> {
        if let Self::Forall(forall) = self {
            Some(forall)
        } else {
            None
        }
    }
}

use super::LambdaOmega;
use errors::TypeMismatch;
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
    fn into_variable(self) -> Result<TypeVariable<LambdaOmega>, TypeMismatch> {
        if let Self::Var(v) = self {
            Ok(v)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
        }
    }

    fn into_unit(self) -> Result<Unit<LambdaOmega>, TypeMismatch> {
        if let Self::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<LambdaOmega>, TypeMismatch> {
        if let Self::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<LambdaOmega>, TypeMismatch> {
        if let Self::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<LambdaOmega>, TypeMismatch> {
        if let Self::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<LambdaOmega>, TypeMismatch> {
        if let Self::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<LambdaOmega>, TypeMismatch> {
        if let Self::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<LambdaOmega>, TypeMismatch> {
        if let Self::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }
}

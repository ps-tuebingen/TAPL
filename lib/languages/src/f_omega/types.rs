use super::FOmega;
use errors::TypeMismatch;
use macros::{
    FromVariants, GrammarDescribe, Kindcheck, LangDisplay, LatexFmt, NoSubtypes, Normalize,
    SubstType,
};
use syntax::types::{
    Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, Type as TypeTrait, TypeGroup,
    TypeVariable, Unit,
};

#[derive(
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
    fn into_fun(self) -> Result<Fun<FOmega>, TypeMismatch> {
        if let Self::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<FOmega>, TypeMismatch> {
        if let Self::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<FOmega>, TypeMismatch> {
        if let Self::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<FOmega>, TypeMismatch> {
        if let Self::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_exists(self) -> Result<Exists<FOmega>, TypeMismatch> {
        if let Self::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<FOmega>, TypeMismatch> {
        if let Self::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<FOmega>, TypeMismatch> {
        if let Self::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_unit(self) -> Result<Unit<FOmega>, TypeMismatch> {
        if let Self::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<FOmega>, TypeMismatch> {
        if let Self::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }
}

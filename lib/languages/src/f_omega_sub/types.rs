use super::FOmegaSub;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, Kindcheck, LangDisplay, LatexFmt, Normalize, SubstType, Subtypecheck};
use syntax::types::{
    ExistsBounded, ForallBounded, Fun, Nat, OpApp, OpLambdaSub, Record, Top, Type as TypeTrait,
    TypeGroup, TypeVariable,
};

#[derive(
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

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<FOmegaSub>::rule(),
            Top::<FOmegaSub>::rule(),
            Fun::<FOmegaSub>::rule(),
            ForallBounded::<FOmegaSub>::rule(),
            OpLambdaSub::<FOmegaSub>::rule(),
            OpApp::<FOmegaSub>::rule(),
            ExistsBounded::<FOmegaSub>::rule(),
            Record::<FOmegaSub>::rule(),
            Nat::<FOmegaSub>::rule(),
        ])
    }
}

impl TypeGroup for Type {
    type Lang = FOmegaSub;
    fn into_variable(self) -> Result<TypeVariable<FOmegaSub>, TypeMismatch> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<FOmegaSub>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<FOmegaSub>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<FOmegaSub>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<FOmegaSub>, TypeMismatch> {
        if let Type::OpLambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpLambda".to_owned()))
        }
    }

    fn into_opapp(self) -> Result<OpApp<FOmegaSub>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.to_string(), "OpApp".to_owned()))
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<FOmegaSub>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<FOmegaSub>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<FOmegaSub>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }
}

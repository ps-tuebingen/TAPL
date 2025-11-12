use super::BoundedQuantification;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt, NoKinds, NoNorm, SubstType, Subtypecheck};
use syntax::types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeGroup, TypeVariable};

#[derive(
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
    fn into_variable(self) -> Result<TypeVariable<BoundedQuantification>, TypeMismatch> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variable".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<BoundedQuantification>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<BoundedQuantification>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<BoundedQuantification>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<BoundedQuantification>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<BoundedQuantification>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<BoundedQuantification>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<BoundedQuantification>::rule(),
            Top::<BoundedQuantification>::rule(),
            Nat::<BoundedQuantification>::rule(),
            Fun::<BoundedQuantification>::rule(),
            ForallBounded::<BoundedQuantification>::rule(),
            ExistsBounded::<BoundedQuantification>::rule(),
            Record::<BoundedQuantification>::rule(),
        ])
    }
}

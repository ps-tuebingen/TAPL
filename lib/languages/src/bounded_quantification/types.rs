use super::BoundedQuantification;
use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use errors::TypeMismatch;
use grammar::{DerivationRule, Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::collections::HashSet;
use std::fmt;
use syntax::{
    TypeVar,
    env::Environment,
    subst::SubstType,
    types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeGroup, TypeVariable},
};

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl From<TypeVariable<BoundedQuantification>> for Type {
    fn from(v: TypeVariable<BoundedQuantification>) -> Type {
        Type::Var(v)
    }
}
impl From<Top<BoundedQuantification>> for Type {
    fn from(t: Top<BoundedQuantification>) -> Type {
        Type::Top(t)
    }
}

impl From<Nat<BoundedQuantification>> for Type {
    fn from(nat: Nat<BoundedQuantification>) -> Type {
        Type::Nat(nat)
    }
}

impl From<Fun<BoundedQuantification>> for Type {
    fn from(fun: Fun<BoundedQuantification>) -> Type {
        Type::Fun(fun)
    }
}

impl From<ForallBounded<BoundedQuantification>> for Type {
    fn from(forall: ForallBounded<BoundedQuantification>) -> Type {
        Type::Forall(forall)
    }
}

impl From<ExistsBounded<BoundedQuantification>> for Type {
    fn from(exists: ExistsBounded<BoundedQuantification>) -> Type {
        Type::Exists(exists)
    }
}

impl From<Record<BoundedQuantification>> for Type {
    fn from(rec: Record<BoundedQuantification>) -> Type {
        Type::Record(rec)
    }
}

impl SubstType for Type {
    type Lang = BoundedQuantification;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(t) => t.subst_type(v, ty).into(),
            Type::Nat(n) => n.subst_type(v, ty).into(),
            Type::Fun(f) => f.subst_type(v, ty).into(),
            Type::Forall(f) => f.subst_type(v, ty).into(),
            Type::Exists(e) => e.subst_type(v, ty).into(),
            Type::Record(rec) => rec.subst_type(v, ty).into(),
        }
    }
}
impl Normalize for Type {
    type Lang = BoundedQuantification;
    fn normalize(self, _: Environment<BoundedQuantification>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => v.fmt(f),
            Type::Top(t) => t.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::Exists(e) => e.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(v) => v.to_latex(conf),
            Type::Top(t) => t.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
            Type::Exists(e) => e.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
        }
    }
}

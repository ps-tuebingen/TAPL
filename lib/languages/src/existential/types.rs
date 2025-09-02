use super::Existential;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Bool, Exists, Fun, Nat, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Existential>),
    Unit(Unit<Existential>),
    Nat(Nat<Existential>),
    Bool(Bool<Existential>),
    Fun(Fun<Existential>),
    Exists(Exists<Existential>),
    Record(Record<Existential>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Existential;
    fn into_unit(self) -> Result<Unit<Existential>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Existential>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Existential>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Existential>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_exists(self) -> Result<Exists<Existential>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(
                self.to_string(),
                "Existential".to_owned(),
            ))
        }
    }

    fn into_record(self) -> Result<Record<Existential>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }
}

impl SubstType for Type {
    type Lang = Existential;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty).into(),
            Type::Unit(u) => u.subst_type(v, ty).into(),
            Type::Nat(nat) => nat.subst_type(v, ty).into(),
            Type::Bool(b) => b.subst_type(v, ty).into(),
            Type::Fun(fun) => fun.subst_type(v, ty).into(),
            Type::Exists(exists) => exists.subst_type(v, ty).into(),
            Type::Record(rec) => rec.subst_type(v, ty).into(),
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Existential>::rule(),
            Unit::<Existential>::rule(),
            Nat::<Existential>::rule(),
            Bool::<Existential>::rule(),
            Fun::<Existential>::rule(),
            Exists::<Existential>::rule(),
            Record::<Existential>::rule(),
        ])
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => v.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Exists(exists) => exists.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(v) => v.to_latex(conf),
            Type::Unit(u) => u.to_latex(conf),
            Type::Nat(nat) => nat.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Exists(exists) => exists.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
        }
    }
}

impl From<Exists<Existential>> for Type {
    fn from(ex: Exists<Existential>) -> Type {
        Type::Exists(ex)
    }
}

impl From<TypeVariable<Existential>> for Type {
    fn from(v: TypeVariable<Existential>) -> Type {
        Type::Var(v)
    }
}
impl From<Unit<Existential>> for Type {
    fn from(u: Unit<Existential>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Existential>> for Type {
    fn from(fun: Fun<Existential>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Existential>> for Type {
    fn from(b: Bool<Existential>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Existential>> for Type {
    fn from(n: Nat<Existential>) -> Type {
        Type::Nat(n)
    }
}

impl From<Record<Existential>> for Type {
    fn from(rec: Record<Existential>) -> Type {
        Type::Record(rec)
    }
}

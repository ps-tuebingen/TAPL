use super::Exceptions;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::NoSubtypes;
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Bool, Fun, Nat, Type as TypeTrait, TypeGroup, Unit},
};

#[derive(NoSubtypes, Debug, Clone, PartialEq, Eq)]
#[Lang(Exceptions)]
pub enum Type {
    Unit(Unit<Exceptions>),
    Nat(Nat<Exceptions>),
    Bool(Bool<Exceptions>),
    Fun(Fun<Exceptions>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Exceptions;
    fn into_unit(self) -> Result<Unit<Exceptions>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_nat(self) -> Result<Nat<Exceptions>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Exceptions>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Exceptions>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Unit::<Exceptions>::rule(),
            Nat::<Exceptions>::rule(),
            Bool::<Exceptions>::rule(),
            Fun::<Exceptions>::rule(),
        ])
    }
}

impl From<Unit<Exceptions>> for Type {
    fn from(u: Unit<Exceptions>) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat<Exceptions>> for Type {
    fn from(nat: Nat<Exceptions>) -> Type {
        Type::Nat(nat)
    }
}

impl From<Bool<Exceptions>> for Type {
    fn from(b: Bool<Exceptions>) -> Type {
        Type::Bool(b)
    }
}

impl From<Fun<Exceptions>> for Type {
    fn from(fun: Fun<Exceptions>) -> Type {
        Type::Fun(fun)
    }
}

impl SubstType for Type {
    type Lang = Exceptions;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Unit(u) => u.subst_type(v, ty).into(),
            Type::Nat(n) => n.subst_type(v, ty).into(),
            Type::Bool(b) => b.subst_type(v, ty).into(),
            Type::Fun(f) => f.subst_type(v, ty).into(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Unit(u) => u.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
        }
    }
}

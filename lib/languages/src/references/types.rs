use super::References;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Bool, Fun, Nat, Reference, Type as TypeTrait, TypeGroup, Unit},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit(Unit<References>),
    Nat(Nat<References>),
    Bool(Bool<References>),
    Fun(Fun<References>),
    Ref(Reference<References>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = References;
    fn into_unit(self) -> Result<Unit<References>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_nat(self) -> Result<Nat<References>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<References>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<References>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_ref(self) -> Result<Reference<References>, TypeMismatch> {
        if let Type::Ref(reft) = self {
            Ok(reft)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Unit::<References>::rule(),
            Nat::<References>::rule(),
            Bool::<References>::rule(),
            Fun::<References>::rule(),
            Reference::<References>::rule(),
        ])
    }
}

impl SubstType for Type {
    type Lang = References;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Ref(rf) => rf.fmt(f),
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
            Type::Ref(rf) => rf.to_latex(conf),
        }
    }
}

impl From<Unit<References>> for Type {
    fn from(u: Unit<References>) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat<References>> for Type {
    fn from(n: Nat<References>) -> Type {
        Type::Nat(n)
    }
}

impl From<Fun<References>> for Type {
    fn from(fun: Fun<References>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Reference<References>> for Type {
    fn from(rf: Reference<References>) -> Type {
        Type::Ref(rf)
    }
}

impl From<Bool<References>> for Type {
    fn from(b: Bool<References>) -> Type {
        Type::Bool(b)
    }
}

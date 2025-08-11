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
    Unit(Unit<Type>),
    Nat(Nat<Type>),
    Bool(Bool<Type>),
    Fun(Fun<Type>),
    Ref(Reference<Type>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Self>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Self>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_ref(self) -> Result<Reference<Self>, TypeMismatch> {
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
            Unit::<Type>::rule(),
            Nat::<Type>::rule(),
            Bool::<Type>::rule(),
            Fun::<Type>::rule(),
            Reference::<Type>::rule(),
        ])
    }
}

impl SubstType<Type> for Type {
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

impl From<Unit<Type>> for Type {
    fn from(u: Unit<Type>) -> Type {
        Type::Unit(u)
    }
}

impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Reference<Type>> for Type {
    fn from(rf: Reference<Type>) -> Type {
        Type::Ref(rf)
    }
}

impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}

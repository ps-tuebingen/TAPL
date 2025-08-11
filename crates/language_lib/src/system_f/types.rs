use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Forall, Fun, Type as TypeTrait, TypeGroup, TypeVariable},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    fn into_fun(self) -> Result<Fun<Self>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_forall(self) -> Result<Forall<Self>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Universal".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Type>::rule(),
            Fun::<Type>::rule(),
            Forall::<Type>::rule(),
        ])
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Var(var) => var.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Forall(forall) => forall.to_latex(conf),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;

    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
        }
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(v: TypeVariable<Type>) -> Type {
        Type::Var(v)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Forall<Type>> for Type {
    fn from(forall: Forall<Type>) -> Type {
        Type::Forall(forall)
    }
}

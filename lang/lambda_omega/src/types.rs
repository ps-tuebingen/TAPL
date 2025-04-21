use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Forall, Fun, Nat, OpApp, OpLambda, TypeVariable, Unit},
};
use std::fmt;

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Unit(Unit<Type>),
    Nat(Nat<Type>),
    Bool(Bool<Type>),
    OpLambda(OpLambda<Type>),
    OpApp(OpApp<Type>),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::OpLambda(oplam) => oplam.fmt(f),
            Type::OpApp(opapp) => opapp.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Unit(u) => u.subst_type(v, ty),
            Type::Bool(b) => b.subst_type(v, ty),
            Type::Nat(n) => n.subst_type(v, ty),
            Type::OpLambda(oplam) => oplam.subst_type(v, ty),
            Type::OpApp(opapp) => opapp.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
        }
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(var: TypeVariable<Type>) -> Type {
        Type::Var(var)
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
impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}
impl From<OpLambda<Type>> for Type {
    fn from(lam: OpLambda<Type>) -> Type {
        Type::OpLambda(lam)
    }
}
impl From<OpApp<Type>> for Type {
    fn from(app: OpApp<Type>) -> Type {
        Type::OpApp(app)
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

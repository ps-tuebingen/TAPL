use common::{
    language::LanguageType,
    subst::SubstType,
    types::{ExistsBounded, ForallBounded, Fun, Nat, OpApp, OpLambda, Record, Top, TypeVariable},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Top(Top<Type>),
    Fun(Fun<Type>),
    Forall(ForallBounded<Type>),
    OpLambda(OpLambda<Type>),
    OpApp(OpApp<Type>),
    Exists(ExistsBounded<Type>),
    Record(Record<Type>),
    Nat(Nat<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(top) => top.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
            Type::OpLambda(lam) => lam.subst_type(v, ty),
            Type::OpApp(app) => app.subst_type(v, ty),
            Type::Exists(ex) => ex.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
            Type::Nat(nat) => nat.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::Top(top) => top.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::OpLambda(lam) => lam.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Exists(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
        }
    }
}
impl From<Top<Type>> for Type {
    fn from(t: Top<Type>) -> Type {
        Type::Top(t)
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(var: TypeVariable<Type>) -> Type {
        Type::Var(var)
    }
}
impl From<OpLambda<Type>> for Type {
    fn from(oplam: OpLambda<Type>) -> Type {
        Type::OpLambda(oplam)
    }
}
impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}
impl From<OpApp<Type>> for Type {
    fn from(opapp: OpApp<Type>) -> Type {
        Type::OpApp(opapp)
    }
}
impl From<ForallBounded<Type>> for Type {
    fn from(forall: ForallBounded<Type>) -> Type {
        Type::Forall(forall)
    }
}
impl From<ExistsBounded<Type>> for Type {
    fn from(exists: ExistsBounded<Type>) -> Type {
        Type::Exists(exists)
    }
}
impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}
impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

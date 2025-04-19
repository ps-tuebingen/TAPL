use common::{
    language::LanguageType,
    subst::SubstType,
    types::{Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, TypeVariable, Unit},
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
    OpLambda(OpLambda<Type>),
    OpApp(OpApp<Type>),
    Exists(Exists<Type>),
    Record(Record<Type>),
    Bool(Bool),
    Unit(Unit),
    Nat(Nat),
}

impl common::types::Type for Type {}

impl LanguageType for Type {}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_ty(v, ty),
            Type::Forall(forall) => forall.subst_ty(v, ty),
            Type::OpLambda(lam) => lam.subst_ty(v, ty),
            Type::OpApp(app) => app.subst_ty(v, ty),
            Type::Exists(ex) => ex.subst_ty(v, ty),
            Type::Record(rec) => rec.subst_ty(v, ty),
            Type::Bool(b) => b.subst_type(v, ty),
            Type::Unit(u) => u.subst_type(v, ty),
            Type::Nat(nat) => nat.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(var) => var.fmt(f),
            Type::OpLambda(lambda) => lambda.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Forall(forall) => forall.fmt(f),
            Type::Exists(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
        }
    }
}

impl From<TypeVariable> for Type {
    fn from(var: TypeVariable) -> Type {
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
impl From<Forall<Type>> for Type {
    fn from(forall: Forall<Type>) -> Type {
        Type::Forall(forall)
    }
}
impl From<Exists<Type>> for Type {
    fn from(exists: Exists<Type>) -> Type {
        Type::Exists(exists)
    }
}
impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}
impl From<Bool> for Type {
    fn from(b: Bool) -> Type {
        Type::Bool(b)
    }
}
impl From<Unit> for Type {
    fn from(u: Uni) -> Type {
        Type::Unit(u)
    }
}
impl From<Nat> for Type {
    fn from(n: Nat) -> Type {
        Type::Nat(n)
    }
}

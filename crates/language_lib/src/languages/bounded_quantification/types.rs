use super::check::Env;
use check::Normalize;
use common::errors::ErrorKind;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{ExistsBounded, ForallBounded, Fun, Nat, Record, Top, TypeGroup, TypeVariable},
    TypeVar,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Top(Top<Type>),
    Nat(Nat<Type>),
    Fun(Fun<Type>),
    Forall(ForallBounded<Type>),
    Exists(ExistsBounded<Type>),
    Record(Record<Type>),
}

impl syntax::types::Type for Type {}

impl TypeGroup for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, ErrorKind> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Type Variable".to_owned(),
            })
        }
    }
    fn into_top(self) -> Result<Top<Type>, ErrorKind> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Top".to_owned(),
            })
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, ErrorKind> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Nat".to_owned(),
            })
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, ErrorKind> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Type>, ErrorKind> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Universal Type".to_owned(),
            })
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<Type>, ErrorKind> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Existential Type".to_owned(),
            })
        }
    }

    fn into_record(self) -> Result<Record<Type>, ErrorKind> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Record Type".to_owned(),
            })
        }
    }
}

impl From<TypeVariable<Type>> for Type {
    fn from(v: TypeVariable<Type>) -> Type {
        Type::Var(v)
    }
}
impl From<Top<Type>> for Type {
    fn from(t: Top<Type>) -> Type {
        Type::Top(t)
    }
}

impl From<Nat<Type>> for Type {
    fn from(nat: Nat<Type>) -> Type {
        Type::Nat(nat)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
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

impl SubstType<Self> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Self) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(t) => t.subst_type(v, ty),
            Type::Nat(n) => n.subst_type(v, ty),
            Type::Fun(f) => f.subst_type(v, ty),
            Type::Forall(f) => f.subst_type(v, ty),
            Type::Exists(e) => e.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
        }
    }
}
impl Normalize<Type> for Type {
    type Env = Env;
    fn normalize(self, _: &mut Self::Env) -> Type {
        self
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

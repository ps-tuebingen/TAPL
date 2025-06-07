use common::errors::{TypeKind, TypeMismatch};
use derivation::latex::LatexFmt;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        ExistsBounded, ForallBounded, Fun, Nat, OpApp, OpLambdaSub, Record, Top, Type as TypeTrait,
        TypeGroup, TypeVariable,
    },
    TypeVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Top(Top<Type>),
    Fun(Fun<Type>),
    Forall(ForallBounded<Type>),
    OpLambdaSub(OpLambdaSub<Type>),
    OpApp(OpApp<Type>),
    Exists(ExistsBounded<Type>),
    Record(Record<Type>),
    Nat(Nat<Type>),
}

impl TypeTrait for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::Var(t) => t.knd(),
            Type::Top(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Forall(t) => t.knd(),
            Type::OpLambdaSub(t) => t.knd(),
            Type::OpApp(t) => t.knd(),
            Type::Exists(t) => t.knd(),
            Type::Record(t) => t.knd(),
            Type::Nat(t) => t.knd(),
        }
    }
}

impl TypeGroup for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, TypeMismatch> {
        if let Type::Var(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Variable))
        }
    }
    fn into_top(self) -> Result<Top<Type>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Top))
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Type>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
        }
    }

    fn into_oplambdasub(self) -> Result<OpLambdaSub<Type>, TypeMismatch> {
        if let Type::OpLambdaSub(lam) = self {
            Ok(lam)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::OpLambda))
        }
    }

    fn into_opapp(self) -> Result<OpApp<Type>, TypeMismatch> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::OpApp))
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<Type>, TypeMismatch> {
        if let Type::Exists(ex) = self {
            Ok(ex)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Existential))
        }
    }

    fn into_record(self) -> Result<Record<Type>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Record))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Type::Var(var) => var.subst_type(v, ty),
            Type::Top(top) => top.subst_type(v, ty),
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
            Type::OpLambdaSub(lam) => lam.subst_type(v, ty),
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
            Type::OpLambdaSub(lam) => lam.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Exists(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self) -> String {
        match self {
            Type::Var(var) => var.to_latex(),
            Type::Top(top) => top.to_latex(),
            Type::Fun(fun) => fun.to_latex(),
            Type::Forall(forall) => forall.to_latex(),
            Type::OpLambdaSub(lam) => lam.to_latex(),
            Type::OpApp(app) => app.to_latex(),
            Type::Exists(ex) => ex.to_latex(),
            Type::Record(rec) => rec.to_latex(),
            Type::Nat(nat) => nat.to_latex(),
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
impl From<OpLambdaSub<Type>> for Type {
    fn from(oplam: OpLambdaSub<Type>) -> Type {
        Type::OpLambdaSub(oplam)
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

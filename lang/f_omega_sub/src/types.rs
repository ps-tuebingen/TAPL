use common::{
    errors::ErrorKind,
    eval::Normalize,
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

impl LanguageType for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Var(var) = self_norm {
            Ok(var)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Type Variable".to_owned(),
            })
        }
    }
    fn into_top(self) -> Result<Top<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Top(top) = self_norm {
            Ok(top)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Top".to_owned(),
            })
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Fun(fun) = self_norm {
            Ok(fun)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }

    fn into_forall_bounded(self) -> Result<ForallBounded<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Forall(forall) = self_norm {
            Ok(forall)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Universal Type".to_owned(),
            })
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::OpLambda(lam) = self_norm {
            Ok(lam)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Operator Abstraction".to_owned(),
            })
        }
    }

    fn into_opapp(self) -> Result<OpApp<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::OpApp(app) = self_norm {
            Ok(app)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Operator Abstraction".to_owned(),
            })
        }
    }

    fn into_exists_bounded(self) -> Result<ExistsBounded<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Exists(ex) = self_norm {
            Ok(ex)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Existential Type".to_owned(),
            })
        }
    }

    fn into_record(self) -> Result<Record<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Record(rec) = self_norm {
            Ok(rec)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Record Type".to_owned(),
            })
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, ErrorKind> {
        let self_norm = self.normalize();
        if let Type::Nat(nat) = self_norm {
            Ok(nat)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self_norm.to_string(),
                expected: "Nat".to_owned(),
            })
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

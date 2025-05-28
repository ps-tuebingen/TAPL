use common::errors::ErrorKind;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{Bool, Exists, Forall, Fun, Nat, OpApp, OpLambda, Record, TypeVariable, Unit},
    TypeVar,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVariable<Type>),
    Fun(Fun<Type>),
    Forall(Forall<Type>),
    OpLambda(OpLambda<Type>),
    OpApp(OpApp<Type>),
    Exists(Exists<Type>),
    Record(Record<Type>),
    Bool(Bool<Type>),
    Unit(Unit<Type>),
    Nat(Nat<Type>),
}

impl common::types::Type for Type {}

impl LanguageType for Type {
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

    fn into_forall(self) -> Result<Forall<Type>, ErrorKind> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Universal Type".to_owned(),
            })
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<Type>, ErrorKind> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Operator Abstraction".to_owned(),
            })
        }
    }

    fn into_opapp(self) -> Result<OpApp<Type>, ErrorKind> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Operator Application".to_owned(),
            })
        }
    }

    fn into_exists(self) -> Result<Exists<Type>, ErrorKind> {
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

    fn into_bool(self) -> Result<Bool<Type>, ErrorKind> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Bool".to_owned(),
            })
        }
    }

    fn into_unit(self) -> Result<Unit<Type>, ErrorKind> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Unit".to_owned(),
            })
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, ErrorKind> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
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
            Type::Fun(fun) => fun.subst_type(v, ty),
            Type::Forall(forall) => forall.subst_type(v, ty),
            Type::OpLambda(lam) => lam.subst_type(v, ty),
            Type::OpApp(app) => app.subst_type(v, ty),
            Type::Exists(ex) => ex.subst_type(v, ty),
            Type::Record(rec) => rec.subst_type(v, ty),
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
impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
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

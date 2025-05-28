use common::errors::ErrorKind;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{Bool, Forall, Fun, Nat, OpApp, OpLambda, TypeVariable, Unit},
};

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

impl LanguageType for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, ErrorKind> {
        if let Type::Var(v) = self {
            Ok(v)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Type Variable".to_owned(),
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
}

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

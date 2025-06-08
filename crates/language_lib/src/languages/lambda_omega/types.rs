use common::errors::{TypeKind, TypeMismatch};
use derivation::latex::LatexFmt;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        Bool, Forall, Fun, Nat, OpApp, OpLambda, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
    },
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

impl TypeTrait for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::Var(t) => t.knd(),
            Type::Unit(t) => t.knd(),
            Type::Nat(t) => t.knd(),
            Type::Bool(t) => t.knd(),
            Type::OpLambda(t) => t.knd(),
            Type::OpApp(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Forall(t) => t.knd(),
        }
    }
}

impl TypeGroup for Type {
    fn into_variable(self) -> Result<TypeVariable<Type>, TypeMismatch> {
        if let Type::Var(v) = self {
            Ok(v)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Variable))
        }
    }

    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Unit))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }

    fn into_bool(self) -> Result<Bool<Type>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Bool))
        }
    }

    fn into_oplambda(self) -> Result<OpLambda<Type>, TypeMismatch> {
        if let Type::OpLambda(lam) = self {
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

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_forall(self) -> Result<Forall<Type>, TypeMismatch> {
        if let Type::Forall(forall) = self {
            Ok(forall)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Universal))
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

impl LatexFmt for Type {
    fn to_latex(&self) -> String {
        match self {
            Type::Var(var) => var.to_latex(),
            Type::Unit(u) => u.to_latex(),
            Type::Bool(b) => b.to_latex(),
            Type::Nat(n) => n.to_latex(),
            Type::OpLambda(oplam) => oplam.to_latex(),
            Type::OpApp(opapp) => opapp.to_latex(),
            Type::Fun(fun) => fun.to_latex(),
            Type::Forall(forall) => forall.to_latex(),
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

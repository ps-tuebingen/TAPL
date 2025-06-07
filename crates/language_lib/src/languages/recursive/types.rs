use common::errors::{TypeKind, TypeMismatch};
use derivation::latex::LatexFmt;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        Bool, Fun, Mu, Nat, Product, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
        Variant,
    },
};

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    TypeVar(TypeVariable<Type>),
    Unit(Unit<Type>),
    Fun(Fun<Type>),
    Mu(Mu<Type>),
    Variant(Variant<Type>),
    Product(Product<Type>),
    Nat(Nat<Type>),
    Bool(Bool<Type>),
    Record(Record<Type>),
}

impl TypeTrait for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::TypeVar(t) => t.knd(),
            Type::Unit(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Mu(t) => t.knd(),
            Type::Variant(t) => t.knd(),
            Type::Product(t) => t.knd(),
            Type::Nat(t) => t.knd(),
            Type::Bool(t) => t.knd(),
            Type::Record(t) => t.knd(),
        }
    }
}

impl TypeGroup for Type {
    fn into_unit(self) -> Result<Unit<Self>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Unit))
        }
    }
    fn into_fun(self) -> Result<Fun<Self>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_mu(self) -> Result<Mu<Self>, TypeMismatch> {
        if let Type::Mu(mu) = self {
            Ok(mu)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Mu))
        }
    }

    fn into_variant(self) -> Result<Variant<Self>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Variant))
        }
    }

    fn into_product(self) -> Result<Product<Self>, TypeMismatch> {
        if let Type::Product(prod) = self {
            Ok(prod)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Product))
        }
    }

    fn into_nat(self) -> Result<Nat<Self>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }

    fn into_bool(self) -> Result<Bool<Self>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Bool))
        }
    }

    fn into_record(self) -> Result<Record<Self>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Record))
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::TypeVar(v) => v.fmt(f),
            Type::Unit(u) => u.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Mu(mu) => mu.fmt(f),
            Type::Variant(var) => var.fmt(f),
            Type::Product(prod) => prod.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Record(rec) => rec.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self) -> String {
        match self {
            Type::TypeVar(v) => v.to_latex(),
            Type::Unit(u) => u.to_latex(),
            Type::Fun(fun) => fun.to_latex(),
            Type::Mu(mu) => mu.to_latex(),
            Type::Variant(var) => var.to_latex(),
            Type::Product(prod) => prod.to_latex(),
            Type::Nat(n) => n.to_latex(),
            Type::Bool(b) => b.to_latex(),
            Type::Record(rec) => rec.to_latex(),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, t: &Self) -> Self::Target {
        match self {
            Type::TypeVar(var) => var.subst_type(v, t),
            Type::Unit(u) => u.subst_type(v, t),
            Type::Fun(fun) => fun.subst_type(v, t),
            Type::Mu(mu) => mu.subst_type(v, t),
            Type::Variant(var) => var.subst_type(v, t),
            Type::Product(prod) => prod.subst_type(v, t),
            Type::Nat(n) => n.subst_type(v, t),
            Type::Bool(b) => b.subst_type(v, t),
            Type::Record(rec) => rec.subst_type(v, t),
        }
    }
}

impl From<Mu<Type>> for Type {
    fn from(mu: Mu<Type>) -> Type {
        Type::Mu(mu)
    }
}
impl From<TypeVariable<Type>> for Type {
    fn from(v: TypeVariable<Type>) -> Type {
        Type::TypeVar(v)
    }
}

impl From<Unit<Type>> for Type {
    fn from(u: Unit<Type>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Type>> for Type {
    fn from(fun: Fun<Type>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Type>> for Type {
    fn from(b: Bool<Type>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Type>> for Type {
    fn from(n: Nat<Type>) -> Type {
        Type::Nat(n)
    }
}

impl From<Product<Type>> for Type {
    fn from(prod: Product<Type>) -> Type {
        Type::Product(prod)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl From<Variant<Type>> for Type {
    fn from(var: Variant<Type>) -> Type {
        Type::Variant(var)
    }
}

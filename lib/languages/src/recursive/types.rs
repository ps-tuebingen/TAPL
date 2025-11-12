use super::Recursive;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::{NoKinds, NoSubtypes};
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        Bool, Fun, Mu, Nat, Product, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
        Variant,
    },
};

pub type TypeVar = String;

#[derive(NoKinds, NoSubtypes, Debug, Clone, PartialEq, Eq)]
#[Lang(Recursive)]
pub enum Type {
    TypeVar(TypeVariable<Recursive>),
    Unit(Unit<Recursive>),
    Fun(Fun<Recursive>),
    Mu(Mu<Recursive>),
    Variant(Variant<Recursive>),
    Product(Product<Recursive>),
    Nat(Nat<Recursive>),
    Bool(Bool<Recursive>),
    Record(Record<Recursive>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Recursive;
    fn into_unit(self) -> Result<Unit<Recursive>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_fun(self) -> Result<Fun<Recursive>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_mu(self) -> Result<Mu<Recursive>, TypeMismatch> {
        if let Type::Mu(mu) = self {
            Ok(mu)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Mu".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Recursive>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_product(self) -> Result<Product<Recursive>, TypeMismatch> {
        if let Type::Product(prod) = self {
            Ok(prod)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Product".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Recursive>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Recursive>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Recursive>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            TypeVariable::<Recursive>::rule(),
            Unit::<Recursive>::rule(),
            Fun::<Recursive>::rule(),
            Mu::<Recursive>::rule(),
            Variant::<Recursive>::rule(),
            Product::<Recursive>::rule(),
            Nat::<Recursive>::rule(),
            Bool::<Recursive>::rule(),
            Record::<Recursive>::rule(),
        ])
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
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::TypeVar(v) => v.to_latex(conf),
            Type::Unit(u) => u.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Mu(mu) => mu.to_latex(conf),
            Type::Variant(var) => var.to_latex(conf),
            Type::Product(prod) => prod.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
        }
    }
}

impl SubstType for Type {
    type Lang = Recursive;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, t: &Self) -> Self::Target {
        match self {
            Type::TypeVar(var) => var.subst_type(v, t),
            Type::Unit(u) => u.subst_type(v, t).into(),
            Type::Fun(fun) => fun.subst_type(v, t).into(),
            Type::Mu(mu) => mu.subst_type(v, t).into(),
            Type::Variant(var) => var.subst_type(v, t).into(),
            Type::Product(prod) => prod.subst_type(v, t).into(),
            Type::Nat(n) => n.subst_type(v, t).into(),
            Type::Bool(b) => b.subst_type(v, t).into(),
            Type::Record(rec) => rec.subst_type(v, t).into(),
        }
    }
}

impl From<Mu<Recursive>> for Type {
    fn from(mu: Mu<Recursive>) -> Type {
        Type::Mu(mu)
    }
}
impl From<TypeVariable<Recursive>> for Type {
    fn from(v: TypeVariable<Recursive>) -> Type {
        Type::TypeVar(v)
    }
}

impl From<Unit<Recursive>> for Type {
    fn from(u: Unit<Recursive>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Recursive>> for Type {
    fn from(fun: Fun<Recursive>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Recursive>> for Type {
    fn from(b: Bool<Recursive>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Recursive>> for Type {
    fn from(n: Nat<Recursive>) -> Type {
        Type::Nat(n)
    }
}

impl From<Product<Recursive>> for Type {
    fn from(prod: Product<Recursive>) -> Type {
        Type::Product(prod)
    }
}

impl From<Record<Recursive>> for Type {
    fn from(rec: Record<Recursive>) -> Type {
        Type::Record(rec)
    }
}

impl From<Variant<Recursive>> for Type {
    fn from(var: Variant<Recursive>) -> Type {
        Type::Variant(var)
    }
}

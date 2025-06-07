use common::errors::{TypeKind, TypeMismatch};
use derivation::latex::LatexFmt;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{
        Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Type as TypeTrait, TypeGroup,
        Unit, Variant,
    },
    TypeVar,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Unit(Unit<Type>),
    Fun(Fun<Type>),
    Bool(Bool<Type>),
    Nat(Nat<Type>),
    Prod(Product<Type>),
    Tup(Tuple<Type>),
    Record(Record<Type>),
    Sum(Sum<Type>),
    Variant(Variant<Type>),
    Optional(Optional<Type>),
    List(List<Type>),
}

impl TypeTrait for Type {
    fn knd(&self) -> TypeKind {
        match self {
            Type::Unit(t) => t.knd(),
            Type::Fun(t) => t.knd(),
            Type::Bool(t) => t.knd(),
            Type::Nat(t) => t.knd(),
            Type::Prod(t) => t.knd(),
            Type::Tup(t) => t.knd(),
            Type::Record(t) => t.knd(),
            Type::Sum(t) => t.knd(),
            Type::Variant(t) => t.knd(),
            Type::Optional(t) => t.knd(),
            Type::List(t) => t.knd(),
        }
    }
}

impl TypeGroup for Type {
    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Unit))
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Function))
        }
    }

    fn into_bool(self) -> Result<Bool<Type>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Bool))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Nat))
        }
    }

    fn into_product(self) -> Result<Product<Type>, TypeMismatch> {
        if let Type::Prod(prod) = self {
            Ok(prod)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Product))
        }
    }

    fn into_tuple(self) -> Result<Tuple<Type>, TypeMismatch> {
        if let Type::Tup(tup) = self {
            Ok(tup)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Tuple))
        }
    }

    fn into_record(self) -> Result<Record<Type>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Record))
        }
    }

    fn into_sum(self) -> Result<Sum<Type>, TypeMismatch> {
        if let Type::Sum(sum) = self {
            Ok(sum)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Sum))
        }
    }

    fn into_variant(self) -> Result<Variant<Type>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Variant))
        }
    }

    fn into_optional(self) -> Result<Optional<Type>, TypeMismatch> {
        if let Type::Optional(opt) = self {
            Ok(opt)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::Option))
        }
    }

    fn into_list(self) -> Result<List<Type>, TypeMismatch> {
        if let Type::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.knd(), TypeKind::List))
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit(u) => u.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Bool(b) => b.fmt(f),
            Type::Nat(n) => n.fmt(f),
            Type::Prod(prod) => prod.fmt(f),
            Type::Tup(tup) => tup.fmt(f),
            Type::Record(recs) => recs.fmt(f),
            Type::Sum(sum) => sum.fmt(f),
            Type::Variant(vars) => vars.fmt(f),
            Type::Optional(option) => option.fmt(f),
            Type::List(list) => list.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self) -> String {
        match self {
            Type::Unit(u) => u.to_latex(),
            Type::Fun(fun) => fun.to_latex(),
            Type::Bool(b) => b.to_latex(),
            Type::Nat(n) => n.to_latex(),
            Type::Prod(prod) => prod.to_latex(),
            Type::Tup(tup) => tup.to_latex(),
            Type::Record(recs) => recs.to_latex(),
            Type::Sum(sum) => sum.to_latex(),
            Type::Variant(vars) => vars.to_latex(),
            Type::Optional(option) => option.to_latex(),
            Type::List(list) => list.to_latex(),
        }
    }
}

impl SubstType<Type> for Type {
    type Target = Type;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
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
        Type::Prod(prod)
    }
}

impl From<Tuple<Type>> for Type {
    fn from(tup: Tuple<Type>) -> Type {
        Type::Tup(tup)
    }
}

impl From<Record<Type>> for Type {
    fn from(rec: Record<Type>) -> Type {
        Type::Record(rec)
    }
}

impl From<Sum<Type>> for Type {
    fn from(sum: Sum<Type>) -> Type {
        Type::Sum(sum)
    }
}

impl From<Variant<Type>> for Type {
    fn from(var: Variant<Type>) -> Type {
        Type::Variant(var)
    }
}

impl From<Optional<Type>> for Type {
    fn from(opt: Optional<Type>) -> Type {
        Type::Optional(opt)
    }
}

impl From<List<Type>> for Type {
    fn from(ls: List<Type>) -> Type {
        Type::List(ls)
    }
}

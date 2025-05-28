use common::errors::ErrorKind;
use std::fmt;
use syntax::{
    subst::SubstType,
    types::{Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Unit, Variant},
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

impl common::types::Type for Type {}

impl LanguageType for Type {
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

    fn into_product(self) -> Result<Product<Type>, ErrorKind> {
        if let Type::Prod(prod) = self {
            Ok(prod)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Product".to_owned(),
            })
        }
    }

    fn into_tuple(self) -> Result<Tuple<Type>, ErrorKind> {
        if let Type::Tup(tup) = self {
            Ok(tup)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Tuple".to_owned(),
            })
        }
    }

    fn into_record(self) -> Result<Record<Type>, ErrorKind> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Record".to_owned(),
            })
        }
    }

    fn into_sum(self) -> Result<Sum<Type>, ErrorKind> {
        if let Type::Sum(sum) = self {
            Ok(sum)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Sum".to_owned(),
            })
        }
    }

    fn into_variant(self) -> Result<Variant<Type>, ErrorKind> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Variant".to_owned(),
            })
        }
    }

    fn into_optional(self) -> Result<Optional<Type>, ErrorKind> {
        if let Type::Optional(opt) = self {
            Ok(opt)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Option".to_owned(),
            })
        }
    }

    fn into_list(self) -> Result<List<Type>, ErrorKind> {
        if let Type::List(list) = self {
            Ok(list)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "List".to_owned(),
            })
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

use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Type as TypeTrait, TypeGroup,
        Unit, Variant,
    },
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

impl TypeTrait for Type {}

impl TypeGroup for Type {
    fn into_unit(self) -> Result<Unit<Type>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Type>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Type>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Type>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_product(self) -> Result<Product<Type>, TypeMismatch> {
        if let Type::Prod(prod) = self {
            Ok(prod)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Product".to_owned()))
        }
    }

    fn into_tuple(self) -> Result<Tuple<Type>, TypeMismatch> {
        if let Type::Tup(tup) = self {
            Ok(tup)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Tuple".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Type>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_sum(self) -> Result<Sum<Type>, TypeMismatch> {
        if let Type::Sum(sum) = self {
            Ok(sum)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Sum".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Type>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_optional(self) -> Result<Optional<Type>, TypeMismatch> {
        if let Type::Optional(opt) = self {
            Ok(opt)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Option".to_owned()))
        }
    }

    fn into_list(self) -> Result<List<Type>, TypeMismatch> {
        if let Type::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Unit::<Type>::rule(),
            Fun::<Type>::rule(),
            Bool::<Type>::rule(),
            Nat::<Type>::rule(),
            Product::<Type>::rule(),
            Tuple::<Type>::rule(),
            Record::<Type>::rule(),
            Sum::<Type>::rule(),
            Variant::<Type>::rule(),
            Optional::<Type>::rule(),
            List::<Type>::rule(),
        ])
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
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Unit(u) => u.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
            Type::Nat(n) => n.to_latex(conf),
            Type::Prod(prod) => prod.to_latex(conf),
            Type::Tup(tup) => tup.to_latex(conf),
            Type::Record(recs) => recs.to_latex(conf),
            Type::Sum(sum) => sum.to_latex(conf),
            Type::Variant(vars) => vars.to_latex(conf),
            Type::Optional(option) => option.to_latex(conf),
            Type::List(list) => list.to_latex(conf),
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

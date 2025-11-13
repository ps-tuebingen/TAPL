use super::Stlc;
use errors::TypeMismatch;
use macros::{
    FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType,
};
use syntax::types::{
    Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Type as TypeTrait, TypeGroup,
    Unit, Variant,
};

#[derive(
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    PartialEq,
    Eq,
    Clone,
    Debug,
)]
#[Lang(Stlc)]
pub enum Type {
    Unit(Unit<Stlc>),
    Fun(Fun<Stlc>),
    Bool(Bool<Stlc>),
    Nat(Nat<Stlc>),
    Prod(Product<Stlc>),
    Tup(Tuple<Stlc>),
    Record(Record<Stlc>),
    Sum(Sum<Stlc>),
    Variant(Variant<Stlc>),
    Optional(Optional<Stlc>),
    List(List<Stlc>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Stlc;
    fn into_unit(self) -> Result<Unit<Stlc>, TypeMismatch> {
        if let Self::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Stlc>, TypeMismatch> {
        if let Self::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Stlc>, TypeMismatch> {
        if let Self::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Stlc>, TypeMismatch> {
        if let Self::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_product(self) -> Result<Product<Stlc>, TypeMismatch> {
        if let Self::Prod(prod) = self {
            Ok(prod)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Product".to_owned()))
        }
    }

    fn into_tuple(self) -> Result<Tuple<Stlc>, TypeMismatch> {
        if let Self::Tup(tup) = self {
            Ok(tup)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Tuple".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Stlc>, TypeMismatch> {
        if let Self::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_sum(self) -> Result<Sum<Stlc>, TypeMismatch> {
        if let Self::Sum(sum) = self {
            Ok(sum)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Sum".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Stlc>, TypeMismatch> {
        if let Self::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_optional(self) -> Result<Optional<Stlc>, TypeMismatch> {
        if let Self::Optional(opt) = self {
            Ok(opt)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Option".to_owned()))
        }
    }

    fn into_list(self) -> Result<List<Stlc>, TypeMismatch> {
        if let Self::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
        }
    }
}

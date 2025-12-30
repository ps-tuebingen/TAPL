use super::Stlc;
use macros::{
    FreeTypeVars, FromVariants, GenerateConstraintsType, GrammarDescribe, LangDisplay, LatexFmt,
    NoKinds, NoNorm, NoSubtypes, SolveConstraint, Spanned, SubstType,
};
use syntax::types::{
    Bool, Fun, List, Nat, Optional, Product, Record, Sum, Tuple, Type as TypeTrait, TypeGroup,
    TypeVariable, Unit, Variant,
};

#[derive(
    SolveConstraint,
    GenerateConstraintsType,
    FreeTypeVars,
    Spanned,
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
    Variable(TypeVariable<Stlc>),
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
    fn into_unit(self) -> Option<Unit<Stlc>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<Stlc>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<Stlc>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<Stlc>> {
        if let Self::Nat(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_product(self) -> Option<Product<Stlc>> {
        if let Self::Prod(prod) = self {
            Some(prod)
        } else {
            None
        }
    }

    fn into_tuple(self) -> Option<Tuple<Stlc>> {
        if let Self::Tup(tup) = self {
            Some(tup)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Stlc>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_sum(self) -> Option<Sum<Stlc>> {
        if let Self::Sum(sum) = self {
            Some(sum)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Stlc>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }

    fn into_optional(self) -> Option<Optional<Stlc>> {
        if let Self::Optional(opt) = self {
            Some(opt)
        } else {
            None
        }
    }

    fn into_list(self) -> Option<List<Stlc>> {
        if let Self::List(list) = self {
            Some(list)
        } else {
            None
        }
    }
}

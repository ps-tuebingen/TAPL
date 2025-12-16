use super::Recursive;
use macros::{
    FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, Spanned,
    SubstType,
};
use syntax::types::{
    Bool, Fun, Mu, Nat, Product, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit, Variant,
};

#[derive(
    Spanned,
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
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
    fn into_unit(self) -> Option<Unit<Recursive>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }
    fn into_fun(self) -> Option<Fun<Recursive>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_mu(self) -> Option<Mu<Recursive>> {
        if let Self::Mu(mu) = self {
            Some(mu)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Recursive>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }

    fn into_product(self) -> Option<Product<Recursive>> {
        if let Self::Product(prod) = self {
            Some(prod)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<Recursive>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<Recursive>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Recursive>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }
}

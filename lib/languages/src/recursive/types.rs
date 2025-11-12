use super::Recursive;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType};
use syntax::types::{
    Bool, Fun, Mu, Nat, Product, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit, Variant,
};

#[derive(
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

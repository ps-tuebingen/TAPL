use super::TypedArithmetic;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType};
use syntax::types::{Bool, Nat, Type as TypeTrait, TypeGroup};

#[derive(
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    Debug,
    PartialEq,
    Eq,
    Clone,
)]
#[Lang(TypedArithmetic)]
pub enum Type {
    Nat(Nat<TypedArithmetic>),
    Bool(Bool<TypedArithmetic>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = TypedArithmetic;
    fn into_nat(self) -> Result<Nat<TypedArithmetic>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<TypedArithmetic>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Nat::<TypedArithmetic>::rule(),
            Bool::<TypedArithmetic>::rule(),
        ])
    }
}

use super::TypedArithmetic;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::{NoKinds, NoNorm, NoSubtypes};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{Bool, Nat, Type as TypeTrait, TypeGroup},
};

#[derive(NoNorm, NoKinds, NoSubtypes, Debug, PartialEq, Eq, Clone)]
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

impl SubstType for Type {
    type Lang = TypedArithmetic;
    type Target = Type;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Nat(n) => n.fmt(f),
            Type::Bool(b) => b.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Nat(n) => n.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
        }
    }
}

impl From<Nat<TypedArithmetic>> for Type {
    fn from(n: Nat<TypedArithmetic>) -> Type {
        Type::Nat(n)
    }
}

impl From<Bool<TypedArithmetic>> for Type {
    fn from(b: Bool<TypedArithmetic>) -> Type {
        Type::Bool(b)
    }
}

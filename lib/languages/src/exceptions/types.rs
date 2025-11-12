use super::Exceptions;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType};
use syntax::types::{Bool, Fun, Nat, Type as TypeTrait, TypeGroup, Unit};

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
#[Lang(Exceptions)]
pub enum Type {
    Unit(Unit<Exceptions>),
    Nat(Nat<Exceptions>),
    Bool(Bool<Exceptions>),
    Fun(Fun<Exceptions>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Exceptions;
    fn into_unit(self) -> Result<Unit<Exceptions>, TypeMismatch> {
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_nat(self) -> Result<Nat<Exceptions>, TypeMismatch> {
        if let Type::Nat(n) = self {
            Ok(n)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Exceptions>, TypeMismatch> {
        if let Type::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Exceptions>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Unit::<Exceptions>::rule(),
            Nat::<Exceptions>::rule(),
            Bool::<Exceptions>::rule(),
            Fun::<Exceptions>::rule(),
        ])
    }
}

#[cfg(test)]
mod type_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use check::Typecheck;
    use syntax::types::Unit;

    #[test]
    fn check1() {
        let result = example_term1().check(Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ret_ty(), expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ret_ty(), expected)
    }
}

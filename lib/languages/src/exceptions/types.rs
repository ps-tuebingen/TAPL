use super::Exceptions;
use macros::{
    FreeTypeVars, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm,
    NoSubtypes, Spanned, SubstType,
};
use syntax::types::{Bool, Fun, Nat, Type as TypeTrait, TypeGroup, Unit};

#[derive(
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
    fn into_unit(self) -> Option<Unit<Exceptions>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }
    fn into_nat(self) -> Option<Nat<Exceptions>> {
        if let Self::Nat(n) = self {
            Some(n)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<Exceptions>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<Exceptions>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod type_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use check::Typecheck;
    use syntax::{span::Span, types::Unit};

    #[test]
    fn check1() {
        let result = example_term1().check(Default::default()).unwrap();
        let expected = Unit::new(Span::default()).into();
        assert_eq!(result.ret_ty(), expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(Default::default()).unwrap();
        let expected = Unit::new(Span::default()).into();
        assert_eq!(result.ret_ty(), expected)
    }
}

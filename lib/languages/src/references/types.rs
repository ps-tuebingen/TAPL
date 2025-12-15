use super::References;
use macros::{
    FromVariants, GrammarDescribe, LangDisplay, LatexFmt, NoKinds, NoNorm, NoSubtypes, SubstType,
};
use syntax::types::{Bool, Fun, Nat, Reference, Type as TypeTrait, TypeGroup, Unit};

#[derive(
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
#[Lang(References)]
pub enum Type {
    Unit(Unit<References>),
    Nat(Nat<References>),
    Bool(Bool<References>),
    Fun(Fun<References>),
    Ref(Reference<References>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = References;
    fn into_unit(self) -> Option<Unit<References>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }
    fn into_nat(self) -> Option<Nat<References>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<References>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<References>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_ref(self) -> Option<Reference<References>> {
        if let Self::Ref(reft) = self {
            Some(reft)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::super::Term;
    use check::Typecheck;
    use syntax::{
        env::Environment,
        span::Span,
        terms::{App, Assign, Deref, Lambda, Loc, Num, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
    };

    #[test]
    fn check1() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Deref::new(Variable::new("x", Span::default()), Span::default()),
                Span::default(),
            ),
            App::new(
                Lambda::new(
                    "y",
                    UnitTy::new(),
                    Ref::new(Variable::new("y", Span::default()), Span::default()),
                    Span::default(),
                ),
                Unit::new(Span::default()),
            ),
        )
        .into();
        let result = term.check(Default::default()).unwrap();
        let expected = UnitTy::new().into();
        assert_eq!(result.ret_ty(), expected)
    }

    #[test]
    fn check2() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Assign::new(
                    Variable::new("x", Span::default()),
                    Deref::new(Variable::new("x", Span::default()), Span::default()),
                ),
                Span::default(),
            ),
            Ref::new(Unit::new(Span::default()), Span::default()),
        )
        .into();
        let result = term.check(Default::default()).unwrap();
        let expected = UnitTy::new().into();
        assert_eq!(result.ret_ty(), expected)
    }

    #[test]
    fn check_fail() {
        let term: Term = App::seq(
            Assign::new(
                Ref::new(Unit::new(Span::default()), Span::default()),
                App::new(
                    Lambda::new(
                        "x",
                        UnitTy::new(),
                        Variable::new("x", Span::default()),
                        Span::default(),
                    ),
                    Unit::new(Span::default()),
                ),
            ),
            Deref::new(Num::new(0, Span::default()), Span::default()),
        )
        .into();
        let result = term.check(Default::default());
        assert!(result.is_err())
    }

    #[test]
    fn check_store() {
        let term: Term = App::seq(
            Assign::new(
                Ref::new(Unit::new(Span::default()), Span::default()),
                App::new(
                    Lambda::new(
                        "x",
                        UnitTy::new(),
                        Variable::new("x", Span::default()),
                        Span::default(),
                    ),
                    Unit::new(Span::default()),
                ),
            ),
            Deref::new(Loc::new(0, Span::default()), Span::default()),
        )
        .into();
        let mut env = Environment::default();
        env.add_loc(0, UnitTy::new().into());
        let result = term.check(env).unwrap();
        let expected = UnitTy::new().into();
        assert_eq!(result.ret_ty(), expected)
    }
}

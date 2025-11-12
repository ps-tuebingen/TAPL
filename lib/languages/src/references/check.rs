use super::{References, types::Type};
use check::{Kindcheck, Subtypecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::{HashMap, HashSet};
use syntax::{Location, Var, env::Environment};

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Location, Type>;

impl Subtypecheck for Type {
    type Lang = References;

    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("References").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = References;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("References").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

#[cfg(test)]
mod check_tests {
    use super::{super::Term, Environment};
    use check::Typecheck;
    use syntax::{
        terms::{App, Assign, Deref, Lambda, Loc, Num, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
    };

    #[test]
    fn check1() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Deref::new(Variable::new("x")),
            ),
            App::new(
                Lambda::new("y", UnitTy::new(), Ref::new(Variable::new("y"))),
                Unit::new(),
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
                Assign::new(Variable::new("x"), Deref::new(Variable::new("x"))),
            ),
            Ref::new(Unit::new()),
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
                Ref::new(Unit::new()),
                App::new(
                    Lambda::new("x", UnitTy::new(), Variable::new("x")),
                    Unit::new(),
                ),
            ),
            Deref::new(Num::new(0)),
        )
        .into();
        let result = term.check(Default::default());
        assert!(result.is_err())
    }

    #[test]
    fn check_store() {
        let term: Term = App::seq(
            Assign::new(
                Ref::new(Unit::new()),
                App::new(
                    Lambda::new("x", UnitTy::new(), Variable::new("x")),
                    Unit::new(),
                ),
            ),
            Deref::new(Loc::new(0)),
        )
        .into();
        let mut env = Environment::default();
        env.add_loc(0, UnitTy::new().into());
        let result = term.check(env).unwrap();
        let expected = UnitTy::new().into();
        assert_eq!(result.ret_ty(), expected)
    }
}

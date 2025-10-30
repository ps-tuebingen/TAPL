use super::{References, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashMap;
use std::collections::HashSet;
use syntax::{
    Location, Var,
    env::Environment,
    terms::{
        App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Loc, Num, Pred, Ref, Succ, True,
        Unit, Variable,
    },
};

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Location, Type>;

impl Typecheck for Term {
    type Lang = References;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Num(c) => c.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::Ref(reft) => reft.check(env),
            Term::Deref(dereft) => dereft.check(env),
            Term::Assign(ass) => ass.check(env),
            Term::Loc(loc) => loc.check(env),
            Term::Let(lett) => lett.check(env),
            Term::If(ift) => ift.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::IsZero(isz) => isz.check(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<References>::rules());
        rules.extend(Num::<References>::rules());
        rules.extend(Succ::<References>::rules());
        rules.extend(Pred::<References>::rules());
        rules.extend(Lambda::<References>::rules());
        rules.extend(App::<References>::rules());
        rules.extend(Unit::<References>::rules());
        rules.extend(Ref::<References>::rules());
        rules.extend(Deref::<References>::rules());
        rules.extend(Assign::<References>::rules());
        rules.extend(Loc::<References>::rules());
        rules.extend(Let::<References>::rules());
        rules.extend(If::<References>::rules());
        rules.extend(True::<References>::rules());
        rules.extend(False::<References>::rules());
        rules.extend(Fix::<References>::rules());
        rules.extend(IsZero::<References>::rules());
        rules
    }
}

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
    use super::{Environment, Term};
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

use super::{Exceptions, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    terms::{
        App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal,
        Unit, Variable,
    },
};

impl Typecheck for Term {
    type Lang = Exceptions;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Num(num) => num.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::If(ift) => ift.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::Exception(exc) => exc.check(env),
            Term::Try(t) => t.check(env),
            Term::Raise(r) => r.check(env),
            Term::TryWithVal(t) => t.check(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Exceptions>::rules());
        rules.extend(Num::<Exceptions>::rules());
        rules.extend(True::<Exceptions>::rules());
        rules.extend(False::<Exceptions>::rules());
        rules.extend(Succ::<Exceptions>::rules());
        rules.extend(Pred::<Exceptions>::rules());
        rules.extend(IsZero::<Exceptions>::rules());
        rules.extend(If::<Exceptions>::rules());
        rules.extend(Lambda::<Exceptions>::rules());
        rules.extend(App::<Exceptions>::rules());
        rules.extend(Unit::<Exceptions>::rules());
        rules.extend(Exception::<Exceptions>::rules());
        rules.extend(Try::<Exceptions>::rules());
        rules.extend(Raise::<Exceptions>::rules());
        rules.extend(TryWithVal::<Exceptions>::rules());
        rules
    }
}

impl Subtypecheck for Type {
    type Lang = Exceptions;

    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Exceptions").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl Kindcheck for Type {
    type Lang = Exceptions;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Exceptions").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

#[cfg(test)]
mod check_tests {
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

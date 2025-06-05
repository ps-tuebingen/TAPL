use super::{errors::Error, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Type = Type;
    type Term = Term;
    type CheckError = Error;

    fn check(
        &self,
        env: &mut Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Error> {
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
}

impl Subtypecheck<Type> for Type {
    type CheckError = Error;

    fn check_subtype(&self, _: &Self, _: &mut Environment<Type>) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type CheckError = Error;

    fn check_kind(&self, _: &mut Environment<Type>) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

#[cfg(test)]
mod check_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use check::Typecheck;
    use syntax::types::Unit;

    #[test]
    fn check1() {
        let result = example_term1().check(&mut Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ty(), expected)
    }

    #[test]
    fn check2() {
        let result = example_term2().check(&mut Default::default()).unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.ty(), expected)
    }
}

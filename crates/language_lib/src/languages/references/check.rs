use super::{terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::TypingDerivation;
use errors::check_error::CheckError;
use std::collections::HashMap;
use syntax::{Location, Var, env::Environment, kinds::Kind};

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Location, Type>;

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<TypingDerivation<Self::Term, Self::Type>, CheckError> {
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
}

impl Subtypecheck<Type> for Type {
    fn check_subtype(&self, _: &Self, _: Environment<Type>) -> Result<(), CheckError> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
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
        assert_eq!(result.ty(), expected)
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
        assert_eq!(result.ty(), expected)
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
        assert_eq!(result.ty(), expected)
    }
}

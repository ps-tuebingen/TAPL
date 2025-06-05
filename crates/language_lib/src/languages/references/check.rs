use super::{errors::Error, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use std::collections::HashMap;
use syntax::{env::Environment, kinds::Kind, Location, Var};

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Location, Type>;

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type CheckError = Error;

    fn check(
        &self,
        env: &mut Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Error> {
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
        let result = term.check(&mut Default::default()).unwrap();
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
        let result = term.check(&mut Default::default()).unwrap();
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
        let result = term.check(&mut Default::default());
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
        let result = term.check(&mut env).unwrap();
        let expected = UnitTy::new().into();
        assert_eq!(result.ty(), expected)
    }
}

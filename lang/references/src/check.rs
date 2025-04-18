use super::{terms::Term, types::Type};
use common::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    Location, Var,
};
use std::collections::HashMap;

pub type Env = HashMap<Var, Type>;
pub type StoreTy = HashMap<Location, Type>;

#[derive(Default, Clone)]
pub struct Environment {
    pub env: Env,
    pub store_ty: StoreTy,
}

impl CheckEnvironment for Environment {
    type Type = Type;

    fn get_var(&self, v: &Var) -> Result<Type, Error> {
        self.env.get_var(v)
    }

    fn get_loc(&self, loc: &Location) -> Result<Type, Error> {
        self.store_ty
            .get(loc)
            .ok_or(to_check_err(ErrorKind::UndefinedLocation(*loc)))
            .cloned()
    }

    fn add_var(&mut self, v: Var, ty: Type) {
        self.env.add_var(v, ty)
    }
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Environment;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
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
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Environment, Term};
    use common::{
        check::Typecheck,
        terms::{App, Assign, Deref, Lambda, Num, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
    };
    use std::collections::HashMap;

    #[test]
    fn check1() {
        let term: Term = App::new(
            Lambda::new("x", Reference::new(UnitTy), Deref::new(Variable::new("x"))),
            App::new(
                Lambda::new("y", UnitTy, Ref::new(Variable::new("y"))),
                Unit::new(),
            ),
        )
        .into();
        let result = term.check(&mut Default::default()).unwrap();
        let expected = UnitTy.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy),
                Assign::new(Variable::new("x"), Deref::new(Variable::new("x"))),
            ),
            Ref::new(Unit::new()),
        )
        .into();
        let result = term.check(&mut Default::default()).unwrap();
        let expected = UnitTy.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let term: Term = App::seq(
            Assign::new(
                Ref::new(Unit::new()),
                App::new(Lambda::new("x", UnitTy, Variable::new("x")), Unit::new()),
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
                App::new(Lambda::new("x", UnitTy, Variable::new("x")), Unit::new()),
            ),
            Deref::new(Num::new(0)),
        )
        .into();
        let result = term
            .check(&mut Environment {
                env: Default::default(),
                store_ty: HashMap::from([(0, UnitTy.into())]),
            })
            .unwrap();
        let expected = UnitTy.into();
        assert_eq!(result, expected)
    }
}

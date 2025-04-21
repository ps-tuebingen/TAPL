use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Kindcheck, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    Location, TypeVar, Var,
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

    fn get_var(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.env.get_var(v)
    }

    fn get_loc(&self, loc: &Location) -> Result<Type, ErrorKind> {
        self.store_ty
            .get(loc)
            .ok_or(ErrorKind::UndefinedLocation(*loc))
            .cloned()
    }

    fn add_var(&mut self, v: Var, ty: Type) {
        self.env.add_var(v, ty)
    }
    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}
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

impl Subtypecheck<Type> for Type {
    type Env = Env;
    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
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

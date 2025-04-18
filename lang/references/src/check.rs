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
    use super::{Environment, Term, Type};
    use common::Typecheck;
    use std::collections::HashMap;

    #[test]
    fn check1() {
        let result = Term::app(
            Term::lam(
                "x",
                Type::Ref(Box::new(Type::Unit)),
                Term::deref("x".into()),
            ),
            Term::app(
                Term::lam("y", Type::Unit, Term::reft("y".into())),
                Term::Unit,
            ),
        )
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check2() {
        let result = Term::App {
            fun: Box::new(Term::Lambda {
                var: "x".to_owned(),
                annot: Type::Ref(Box::new(Type::Unit)),
                body: Box::new(Term::Assign {
                    to: Box::new(Term::Var("x".to_owned())),
                    body: Box::new(Term::Deref(Box::new(Term::Var("x".to_owned())))),
                }),
            }),
            arg: Box::new(Term::Ref(Box::new(Term::Unit))),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fail() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .check(&mut Default::default());
        assert!(result.is_err())
    }

    #[test]
    fn check_store() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .check(&mut Environment {
            env: Default::default(),
            store_ty: HashMap::from([(0, Type::Unit)]),
        })
        .unwrap();
        let expected = Type::Unit;
        assert_eq!(result, expected)
    }
}

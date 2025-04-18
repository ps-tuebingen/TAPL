use super::{terms::Term, values::Value};
use common::{
    errors::{Error, ErrorKind},
    eval::{Eval, EvalEnvironment},
    Location,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Store(HashMap<Location, Value>);

impl EvalEnvironment for Store {
    fn fresh_location(&self) -> Location {
        let mut next_loc = 0;
        while self.contains_key(&next_loc) {
            next_loc += 1;
        }
        next_loc
    }
}

impl Eval for Term {
    type Value = Value;
    type Env = Store;

    fn eval(self, st: &mut Store) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Num(c) => c.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Ref(reft) => reft.eval(env),
            Term::Deref(dereft) => dereft.eval(env),
            Term::Assign(ass) => ass.eval(env),
            Term::Loc(loc) => loc.eval(env),
            Term::Let(lett) => lett.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Eval, Term, Value};
    use crate::types::Type;

    #[test]
    fn eval1() {
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
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
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
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_store() {
        let result = Term::seq(
            Term::assign(
                Term::reft(Term::Unit),
                Term::app(Term::lam("x", Type::Unit, "x".into()), Term::Unit),
            ),
            Term::deref(0.into()),
        )
        .eval(&mut Default::default())
        .unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}

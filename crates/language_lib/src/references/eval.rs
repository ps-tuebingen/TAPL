use super::{check::Environment, terms::Term, types::Type, values::Value};
use common::{
    errors::{Error, ErrorKind},
    eval::{Eval, EvalEnvironment, Normalize},
    Location,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct Store(HashMap<Location, Value>);

impl EvalEnvironment<Value> for Store {
    fn fresh_location(&self) -> Location {
        let mut next_loc = 0;
        while self.0.contains_key(&next_loc) {
            next_loc += 1;
        }
        next_loc
    }
    fn get_location(&self, loc: Location) -> Result<Value, ErrorKind> {
        self.0
            .get(&loc)
            .ok_or(ErrorKind::UndefinedLocation(loc))
            .cloned()
    }

    fn save_location(&mut self, loc: Location, val: Value) {
        self.0.insert(loc, val);
    }
}

impl Eval for Term {
    type Value = Value;
    type Env = Store;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
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
            Term::Fix(fix) => fix.eval(env),
            Term::IsZero(isz) => isz.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    type Env = Environment;
    fn normalize(self, _: &mut Self::Env) -> Type {
        self
    }
}

#[cfg(test)]
mod check_tests {
    use super::{Eval, Term};
    use common::{
        terms::{App, Assign, Deref, Lambda, Loc, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
        values::Unit as UnitVal,
    };

    #[test]
    fn eval1() {
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
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Assign::new(Variable::new("x"), Deref::new(Variable::new("x"))),
            ),
            Ref::new(Unit::new()),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_store() {
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
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result, expected)
    }
}

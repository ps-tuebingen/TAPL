use super::terms::Term;
use common::{
    errors::Error,
    language::LanguageValue,
    values::{Lambda, Value as ValueTrait},
    Eval,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
        }
    }
}

impl From<Lambda<Term>> for Value {
    fn from(lam: Lambda<Term>) -> Value {
        Value::Lambda(lam)
    }
}

impl Eval for Term {
    type Value = Value;
    type Env = ();

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::is_value;
    use crate::terms::Term;

    #[test]
    fn is_value_lam() {
        let result = is_value(&Term::Lambda(
            "x".to_owned(),
            Box::new(Term::Var("x".to_owned())),
        ));
        assert!(result)
    }

    #[test]
    fn is_value_var() {
        let result = is_value(&Term::Var("x".to_owned()));
        assert!(!result)
    }
}

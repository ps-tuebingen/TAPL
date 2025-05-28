use super::values::Value;
use common::{
    language::{untyped::Untyped, LanguageTerm},
    subst::SubstTerm,
    terms::{App, Lambda, Variable},
};
use std::fmt;

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term>),
    App(App<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Untyped;
    type Value = Value;
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
        }
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
        }
    }
}

impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
    }
}

impl From<Lambda<Term>> for Term {
    fn from(lam: Lambda<Term>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;
    use common::{
        language::untyped::Untyped,
        subst::SubstTerm,
        terms::{App, Lambda, Variable},
    };

    #[test]
    fn subst1() {
        let result: Term = Lambda::new("x", Untyped, Variable::new("x"))
            .subst(&"x".to_owned(), &Variable::new("y").into());
        let expected = Lambda::new("x", Untyped, Variable::new("x")).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let term: Term = App::new(
            Lambda::new("x", Untyped, Variable::new("y")),
            Variable::new("x"),
        )
        .into();
        let result: Term = term
            .subst(&"x".to_owned(), &Variable::new("z").into())
            .subst(&"y".to_owned(), &Variable::new("z").into());
        let expected = App::new(
            Lambda::new("x", Untyped, Variable::new("z")),
            Variable::new("z"),
        )
        .into();
        assert_eq!(result, expected)
    }
}

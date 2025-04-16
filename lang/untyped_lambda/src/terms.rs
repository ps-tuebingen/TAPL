use super::eval::Value;
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
    use std::collections::HashSet;

    #[test]
    fn free_v1() {
        let result = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned()))).free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_v2() {
        let result = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("y".to_owned())),
            )),
            Box::new(Term::Var("x".to_owned())),
        )
        .free_vars();
        let expected = HashSet::from(["x".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst1() {
        let result = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned())))
            .subst(&"x".to_owned(), Term::Var("y".to_owned()));
        let expected = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned())));
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("y".to_owned())),
            )),
            Box::new(Term::Var("x".to_owned())),
        )
        .subst(&"x".to_owned(), Term::Var("z".to_owned()))
        .subst(&"y".to_owned(), Term::Var("z".to_owned()));
        let expected = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("z".to_owned())),
            )),
            Box::new(Term::Var("z".to_owned())),
        );
        assert_eq!(result, expected)
    }
}

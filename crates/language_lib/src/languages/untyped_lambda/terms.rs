use super::errors::Error;
use check::Typecheck;
use derivation::Derivation;
use std::fmt;
use syntax::untyped::Untyped;
use syntax::{
    env::Environment,
    subst::{SubstTerm, SubstType},
    terms::{App, Lambda, Variable},
    TypeVar,
};

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term, Untyped>),
    App(App<Term>),
}

impl syntax::terms::Term for Term {}

impl Typecheck for Term {
    type Term = Term;
    type Type = Untyped;
    type CheckError = Error;

    fn check(
        &self,
        _: &mut Environment<Untyped>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Error> {
        Ok(Derivation::empty(self.clone()))
    }
}

impl SubstType<Untyped> for Term {
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Untyped) -> Self::Target {
        self
    }
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

impl From<Lambda<Term, Untyped>> for Term {
    fn from(lam: Lambda<Term, Untyped>) -> Term {
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
    use syntax::{
        subst::SubstTerm,
        terms::{App, Lambda, Variable},
        untyped::Untyped,
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

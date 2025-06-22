use check::{Typecheck, errors::CheckError};
use derivation::TypingDerivation;
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::untyped::Untyped;
use syntax::{
    TypeVar,
    env::Environment,
    subst::{SubstTerm, SubstType},
    terms::{App, UntypedLambda, Variable},
};

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(UntypedLambda<Term>),
    App(App<Term>),
}

impl syntax::terms::Term for Term {}

impl Typecheck for Term {
    type Term = Term;
    type Type = Untyped;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, _: Environment<Untyped>) -> Result<Self::Deriv, CheckError> {
        Ok(TypingDerivation::empty(self.clone()))
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
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

impl From<UntypedLambda<Term>> for Term {
    fn from(lam: UntypedLambda<Term>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

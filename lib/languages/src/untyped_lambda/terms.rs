use check::Typecheck;
use derivations::{Derivation, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
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
    type Type = Untyped<Term>;

    fn check(
        &self,
        _: Environment<Untyped<Term>>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        Ok(TypingDerivation::empty(self.clone()).into())
    }
}

impl SubstType<Untyped<Term>> for Term {
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Untyped<Term>) -> Self::Target {
        self
    }
}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Term>::rule(),
            UntypedLambda::<Term>::rule(),
            App::<Term>::rule(),
        ])
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

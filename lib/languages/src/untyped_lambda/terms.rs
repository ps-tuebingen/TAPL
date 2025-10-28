use super::UntypedLambda;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    env::Environment,
    language::Language,
    subst::{SubstTerm, SubstType},
    terms::{App, UntypedLambda as UntypedLambdaT, Variable},
};

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<UntypedLambda>),
    Lambda(UntypedLambdaT<UntypedLambda>),
    App(App<UntypedLambda>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<UntypedLambda>::rule(),
            UntypedLambdaT::<UntypedLambda>::rule(),
            App::<UntypedLambda>::rule(),
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

impl SubstTerm for Term {
    type Lang = UntypedLambda;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = UntypedLambda;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &<Self::Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl Typecheck for Term {
    type Lang = UntypedLambda;

    fn check(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoTyping::new(UntypedLambda.describe()).into())
    }
}

impl From<Variable<UntypedLambda>> for Term {
    fn from(var: Variable<UntypedLambda>) -> Term {
        Term::Var(var)
    }
}

impl From<UntypedLambdaT<UntypedLambda>> for Term {
    fn from(lam: UntypedLambdaT<UntypedLambda>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<UntypedLambda>> for Term {
    fn from(app: App<UntypedLambda>) -> Term {
        Term::App(app)
    }
}

use super::{UntypedLambda, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{App, UntypedLambda as UntypedLambdaT, Variable},
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = UntypedLambda;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<UntypedLambda>::rules());
        rules.extend(UntypedLambdaT::<UntypedLambda>::rules());
        rules.extend(App::<UntypedLambda>::rules());
        rules
    }
}

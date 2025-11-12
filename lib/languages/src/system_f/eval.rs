use super::{SystemF, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = SystemF;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<SystemF>::rules());
        rules.extend(Lambda::<SystemF>::rules());
        rules.extend(Lambda::<SystemF>::rules());
        rules.extend(App::<SystemF>::rules());
        rules.extend(TyLambda::<SystemF>::rules());
        rules.extend(TyApp::<SystemF>::rules());
        rules
    }
}

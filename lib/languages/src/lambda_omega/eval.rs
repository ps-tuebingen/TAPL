use super::{LambdaOmega, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable},
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = LambdaOmega;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Num(num) => num.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::TyLambda(tylam) => tylam.eval(env),
            Term::TyApp(tyapp) => tyapp.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<LambdaOmega>::rules());
        rules.extend(Num::<LambdaOmega>::rules());
        rules.extend(True::<LambdaOmega>::rules());
        rules.extend(False::<LambdaOmega>::rules());
        rules.extend(Lambda::<LambdaOmega>::rules());
        rules.extend(App::<LambdaOmega>::rules());
        rules.extend(Unit::<LambdaOmega>::rules());
        rules.extend(TyLambda::<LambdaOmega>::rules());
        rules.extend(TyApp::<LambdaOmega>::rules());
        rules
    }
}

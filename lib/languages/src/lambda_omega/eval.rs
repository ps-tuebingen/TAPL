use super::{LambdaOmega, terms::Term, types::Type};
use check::Normalize;
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::env::Environment;
use syntax::eval_context::EvalContext;
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
}

impl Normalize for Type {
    type Lang = LambdaOmega;
    fn normalize(self, _: Environment<Self::Lang>) -> Type {
        self
    }
}

use super::{BoundedQuantification, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Lang = BoundedQuantification;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env).into(),
            Term::Num(num) => num.eval(env).into(),
            Term::Succ(succ) => succ.eval(env).into(),
            Term::Pred(pred) => pred.eval(env).into(),
            Term::Lambda(lam) => lam.eval(env).into(),
            Term::App(app) => app.eval(env).into(),
            Term::LambdaSub(lam) => lam.eval(env).into(),
            Term::TyApp(app) => app.eval(env).into(),
            Term::Pack(pack) => pack.eval(env).into(),
            Term::Unpack(unpack) => unpack.eval(env).into(),
            Term::Record(rec) => rec.eval(env).into(),
            Term::Projection(proj) => proj.eval(env).into(),
        }
    }
}

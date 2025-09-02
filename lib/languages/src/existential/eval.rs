use super::{Existential, terms::Term, types::Type};
use check::Normalize;
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::env::Environment;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Lang = Existential;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Fix(fix) => fix.eval(env),
        }
    }
}

impl Normalize for Type {
    type Lang = Existential;
    fn normalize(self, _: Environment<Self::Lang>) -> Type {
        self
    }
}

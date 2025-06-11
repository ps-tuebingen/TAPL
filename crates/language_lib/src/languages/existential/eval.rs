use super::{errors::Error, terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::Eval;
use syntax::env::Environment;
use trace::EvalTrace;

impl Eval for Term {
    type Env = ();
    type Term = Term;
    type Value = Value;
    type EvalError = Error;

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
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

impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}

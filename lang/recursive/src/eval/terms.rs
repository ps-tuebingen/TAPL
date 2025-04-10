use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::Term,
};
use common::Eval;

impl Eval<'_> for Term {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self {
            Term::Var(v) => Err(Error::eval(ErrorKind::FreeVar(v.clone()), &Term::Var(v))),
            Term::Lambda(lam) => lam.eval(_env),
            Term::App(app) => app.eval(_env),
            Term::Unit => Ok(Value::Unit),
            Term::Fold(fold) => fold.eval(_env),
            Term::Unfold(unfold) => unfold.eval(_env),
            Term::Variant(var) => var.eval(_env),
            Term::VariantCase(case) => case.eval(_env),
            Term::Pair(p) => p.eval(_env),
            Term::Fst(fst) => fst.eval(_env),
            Term::Snd(snd) => snd.eval(_env),
            Term::Zero(zero) => zero.eval(_env),
            Term::Succ(succ) => succ.eval(_env),
            Term::Pred(pred) => pred.eval(_env),
            Term::IsZero(isz) => isz.eval(_env),
            Term::True(tru) => tru.eval(_env),
            Term::False(fls) => fls.eval(_env),
            Term::If(ift) => ift.eval(_env),
            Term::Fix(fix) => fix.eval(_env),
            Term::Let(lt) => lt.eval(_env),
            Term::Record(rec) => rec.eval(_env),
            Term::RecordProj(proj) => proj.eval(_env),
        }
    }
}

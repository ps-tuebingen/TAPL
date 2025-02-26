use super::Eval;
use crate::{
    errors::{Error, ErrorKind},
    terms::Term,
};

impl Eval for Term {
    fn eval_once(self) -> Result<Term, Error> {
        match self {
            Term::Var(v) => Err(Error::eval(ErrorKind::FreeVar(v.clone()), &Term::Var(v))),
            Term::Lambda(lam) => lam.eval_once(),
            Term::App(app) => app.eval_once(),
            Term::Unit => Ok(Term::Unit),
            Term::Fold(fold) => fold.eval_once(),
            Term::Unfold(unfold) => unfold.eval_once(),
            Term::Variant(var) => var.eval_once(),
            Term::VariantCase(case) => case.eval_once(),
            Term::Pair(p) => p.eval_once(),
            Term::Fst(fst) => fst.eval_once(),
            Term::Snd(snd) => snd.eval_once(),
            Term::Zero(zero) => zero.eval_once(),
            Term::Succ(succ) => succ.eval_once(),
            Term::Pred(pred) => pred.eval_once(),
            Term::IsZero(isz) => isz.eval_once(),
            Term::True(tru) => tru.eval_once(),
            Term::False(fls) => fls.eval_once(),
            Term::If(ift) => ift.eval_once(),
            Term::Fix(fix) => fix.eval_once(),
            Term::Let(lt) => lt.eval_once(),
            Term::Record(rec) => rec.eval_once(),
            Term::RecordProj(proj) => proj.eval_once(),
        }
    }
}

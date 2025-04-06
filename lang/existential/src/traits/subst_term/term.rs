use super::SubstTerm;
use crate::terms::{Term, Var};

impl SubstTerm for Term {
    fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Unit => Term::Unit,
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::Zero(zero) => zero.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
        }
    }
}

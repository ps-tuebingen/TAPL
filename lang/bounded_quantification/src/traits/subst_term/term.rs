use super::SubstTerm;
use crate::syntax::{Term, Var};

impl SubstTerm for Term {
    fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(var) => {
                if var == *v {
                    t
                } else {
                    Term::Var(var)
                }
            }
            Term::Const(c) => c.subst(v, t),
            Term::Succ(s) => s.subst(v, t),
            Term::Pred(p) => p.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::LambdaSub(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
        }
    }
}

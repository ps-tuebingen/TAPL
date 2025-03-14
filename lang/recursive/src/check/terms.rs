use super::{Check, Env};
use crate::{errors::Error, terms::Term, types::Type};

impl Check for Term {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(v) => env.get(v).map_err(|knd| Error::check(knd, self)),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit => Ok(Type::Unit),
            Term::Fold(fold) => fold.check(env),
            Term::Unfold(unfold) => unfold.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Pair(p) => p.check(env),
            Term::Fst(fst) => fst.check(env),
            Term::Snd(snd) => snd.check(env),
            Term::Zero(zero) => zero.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
        }
    }
}

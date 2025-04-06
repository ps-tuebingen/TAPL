use super::{Check, Env};
use crate::{errors::Error, terms::Term, types::Type};

impl Check for Term {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Unit => Ok(Type::Unit),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Zero(zero) => zero.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

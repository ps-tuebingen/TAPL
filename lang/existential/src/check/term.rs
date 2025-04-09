use super::Env;
use crate::{errors::Error, terms::Term, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(var) => env.get_var(var).map_err(|knd| Error::check(knd, self)),
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

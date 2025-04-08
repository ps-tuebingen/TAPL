use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::Term,
};
use common::Eval;

impl Eval<'_> for Term {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        match self {
            Term::Var(v) => Err(Error::eval(ErrorKind::FreeVar(v.clone()), &v.as_str())),
            Term::Lambda(lam) => lam.eval(_env),
            Term::App(app) => app.eval(_env),
            Term::TyLambda(lam) => lam.eval(_env),
            Term::TyApp(app) => app.eval(_env),
            Term::Pack(pack) => pack.eval(_env),
            Term::Unpack(unpack) => unpack.eval(_env),
            Term::Record(rec) => rec.eval(_env),
            Term::RecordProj(proj) => proj.eval(_env),
            Term::True(tru) => tru.eval(_env),
            Term::False(fls) => fls.eval(_env),
            Term::If(ift) => ift.eval(_env),
            Term::Unit => Ok(Value::Unit),
            Term::Fix(fix) => fix.eval(_env),
            Term::Zero(z) => z.eval(_env),
            Term::Succ(s) => s.eval(_env),
            Term::Pred(p) => p.eval(_env),
            Term::IsZero(isz) => isz.eval(_env),
        }
    }
}

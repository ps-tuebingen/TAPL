use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::Term,
};

impl Eval for Term {
    fn eval(self) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::eval(ErrorKind::FreeVar(v.clone()), &v.as_str())),
            Term::Lambda(lam) => lam.eval(),
            Term::App(app) => app.eval(),
            Term::TyLambda(lam) => lam.eval(),
            Term::TyApp(app) => app.eval(),
            Term::Pack(pack) => pack.eval(),
            Term::Unpack(unpack) => unpack.eval(),
            Term::Record(rec) => rec.eval(),
            Term::RecordProj(proj) => proj.eval(),
            Term::True(tru) => tru.eval(),
            Term::False(fls) => fls.eval(),
            Term::If(ift) => ift.eval(),
            Term::Unit => Ok(Value::Unit),
            Term::Fix(fix) => fix.eval(),
        }
    }
}

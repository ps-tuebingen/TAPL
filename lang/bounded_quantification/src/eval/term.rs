use super::{Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::Term,
};

impl Eval for Term {
    fn eval(self) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::check(ErrorKind::FreeVar(v.clone()), &v.as_str())),
            Term::Const(c) => c.eval(),
            Term::Succ(s) => s.eval(),
            Term::Pred(p) => p.eval(),
            Term::Lambda(lam) => lam.eval(),
            Term::App(app) => app.eval(),
            Term::LambdaSub(lam) => lam.eval(),
            Term::TyApp(app) => app.eval(),
            Term::Pack(pack) => pack.eval(),
            Term::Unpack(unpack) => unpack.eval(),
        }
    }
}

use super::{Env, Eval, Value};
use crate::{
    errors::{Error, ErrorKind},
    syntax::terms::Term,
};

pub mod app;
pub mod lambda;
pub mod let_exp;
pub mod pack;
pub mod pred;
pub mod record;
pub mod recordproj;
pub mod succ;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;

impl Eval for Term {
    type Target = Value;
    fn eval(self, env: &mut Env) -> Result<Self::Target, Error> {
        match self {
            Term::Var(ref v) => Err(Error::eval(ErrorKind::FreeVar(v.clone()), self)),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Zero => Ok(Value::Zero),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::Let(lt) => lt.eval(env),
        }
    }
}

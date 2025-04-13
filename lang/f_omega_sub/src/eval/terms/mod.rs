use super::{to_eval_err, Env, Value};
use crate::syntax::terms::Term;
use common::{
    errors::{Error, ErrorKind},
    Eval,
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

impl<'a> Eval<'a> for Term {
    type Value = Value;
    type Env = &'a mut Env;

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut Default::default())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(ref v) => Err(to_eval_err(ErrorKind::FreeVariable(v.clone()))),
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

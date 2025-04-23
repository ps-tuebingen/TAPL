use super::{terms::Term, types::Type, values::Value};
use common::{
    errors::Error,
    eval::{Eval, Normalize},
};

impl Eval for Term {
    type Env = ();
    type Value = Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::LambdaSub(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::Let(lt) => lt.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self) -> Self {
        match self {
            Type::Var(var) => var.normalize(),
            Type::Top(top) => top.normalize(),
            Type::Fun(fun) => fun.normalize(),
            Type::Forall(forall) => forall.normalize(),
            Type::OpLambdaSub(lam) => lam.normalize(),
            Type::OpApp(app) => app.normalize(),
            Type::Exists(ex) => ex.normalize(),
            Type::Record(rec) => rec.normalize(),
            Type::Nat(nat) => nat.normalize(),
        }
    }
}

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
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::IsZero(isz) => isz.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self) -> Type {
        match self {
            Type::Var(var) => var.normalize(),
            Type::Fun(fun) => fun.normalize(),
            Type::Forall(forall) => forall.normalize(),
            Type::OpLambda(lam) => lam.normalize(),
            Type::OpApp(app) => app.normalize(),
            Type::Exists(ex) => ex.normalize(),
            Type::Record(rec) => rec.normalize(),
            Type::Bool(b) => b.normalize(),
            Type::Unit(u) => u.normalize(),
            Type::Nat(nat) => nat.normalize(),
        }
    }
}

use super::{terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::{Eval, errors::EvalError};
use syntax::env::Environment;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Term = Term;
    type Value = Value;

    fn eval(
        self,
        env: &mut EvalContext<Term, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
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
    fn normalize(self, env: Environment<Type>) -> Self {
        match self {
            Type::Var(var) => var.normalize(env),
            Type::Top(top) => top.normalize(env),
            Type::Fun(fun) => fun.normalize(env),
            Type::Forall(forall) => forall.normalize(env),
            Type::OpLambdaSub(lam) => lam.normalize(env),
            Type::OpApp(app) => app.normalize(env),
            Type::Exists(ex) => ex.normalize(env),
            Type::Record(rec) => rec.normalize(env),
            Type::Nat(nat) => nat.normalize(env),
        }
    }
}

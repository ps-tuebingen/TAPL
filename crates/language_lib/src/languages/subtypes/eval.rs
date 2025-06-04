use super::{errors::Error, terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::Eval;
use syntax::env::Environment;

impl Eval for Term {
    type Env = ();
    type Value = Value;
    type EvalError = Error;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(unit) => unit.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Cast(cast) => cast.eval(env),
            Term::Nil(nil) => nil.eval(env),
            Term::Cons(cons) => cons.eval(env),
            Term::ListCase(case) => case.eval(env),
            Term::Ref(rf) => rf.eval(env),
            Term::Deref(deref) => deref.eval(env),
            Term::Assign(assign) => assign.eval(env),
            Term::Loc(loc) => loc.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Fix(fix) => fix.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self, _: &mut Environment<Type>) -> Type {
        self
    }
}

use super::{errors::Error, terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::Eval;
use syntax::env::Environment;
use trace::EvalTrace;

impl Eval for Term {
    type Env = ();
    type Term = Term;
    type Value = Value;
    type EvalError = Error;

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Fold(fold) => fold.eval(env),
            Term::Unfold(unfold) => unfold.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Pair(p) => p.eval(env),
            Term::Fst(fst) => fst.eval(env),
            Term::Snd(snd) => snd.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}

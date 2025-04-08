use super::{errors::Error, Eval, Value};
use crate::syntax::Term;

impl Eval<'_> for Term {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self {
            Term::Var(v) => Err(Error::FreeVariable { var: v }),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(unit) => unit.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Zero(z) => z.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Ascribe(asc) => asc.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Pair(pr) => pr.eval(env),
            Term::Proj1(proj) => proj.eval(env),
            Term::Proj2(proj) => proj.eval(env),
            Term::Tup(tup) => tup.eval(env),
            Term::Proj(proj) => proj.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Left(lf) => lf.eval(env),
            Term::Right(rt) => rt.eval(env),
            Term::SumCase(case) => case.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Nothing(not) => not.eval(env),
            Term::Something(some) => some.eval(env),
            Term::SomeCase(case) => case.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Nil(nil) => nil.eval(env),
            Term::Cons(cons) => cons.eval(env),
            Term::IsNil(isnil) => isnil.eval(env),
            Term::Head(hd) => hd.eval(env),
            Term::Tail(tl) => tl.eval(env),
        }
    }
}

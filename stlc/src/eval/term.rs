use super::{Eval, Value};
use crate::terms::syntax::Term;

impl Eval for Term {
    fn eval(self) -> Option<Value> {
        match self {
            Term::Var(v) => v.eval(),
            Term::Lambda(lam) => lam.eval(),
            Term::App(app) => app.eval(),
            Term::Unit(unit) => unit.eval(),
            Term::True(tru) => tru.eval(),
            Term::False(fls) => fls.eval(),
            Term::If(ift) => ift.eval(),
            Term::Ascribe(asc) => asc.eval(),
            Term::Let(lt) => lt.eval(),
            Term::Pair(pr) => pr.eval(),
            Term::Proj1(proj) => proj.eval(),
            Term::Proj2(proj) => proj.eval(),
            Term::Tup(tup) => tup.eval(),
            Term::Proj(proj) => proj.eval(),
            Term::Record(rec) => rec.eval(),
            Term::RecordProj(proj) => proj.eval(),
            Term::Left(lf) => lf.eval(),
            Term::Right(rt) => rt.eval(),
            Term::SumCase(case) => case.eval(),
            Term::Variant(var) => var.eval(),
            Term::VariantCase(case) => case.eval(),
            Term::Nothing(not) => not.eval(),
            Term::Something(some) => some.eval(),
            Term::Fix(fix) => fix.eval(),
            Term::Nil(nil) => nil.eval(),
            Term::Cons(cons) => cons.eval(),
            Term::IsNil(isnil) => isnil.eval(),
            Term::Head(hd) => hd.eval(),
            Term::Tail(tl) => tl.eval(),
        }
    }
}
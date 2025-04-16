use super::{AsContext, Error, EvalContext};
use crate::syntax::Term;

impl AsContext for Term {
    fn to_context(self) -> Result<EvalContext, Error> {
        match self {
            Term::Var(v) => v.to_context(),
            Term::Lambda(lam) => lam.to_context(),
            Term::App(app) => app.to_context(),
            Term::Unit(unit) => unit.to_context(),
            Term::True(tru) => tru.to_context(),
            Term::False(fls) => fls.to_context(),
            Term::If(ift) => ift.to_context(),
            Term::Zero(z) => z.to_context(),
            Term::Pred(p) => p.to_context(),
            Term::Succ(s) => s.to_context(),
            Term::IsZero(isz) => isz.to_context(),
            Term::Ascribe(asc) => asc.to_context(),
            Term::Let(lt) => lt.to_context(),
            Term::Pair(pr) => pr.to_context(),
            Term::Tup(tup) => tup.to_context(),
            Term::Proj(proj) => proj.to_context(),
            Term::Proj1(proj) => proj.to_context(),
            Term::Proj2(proj) => proj.to_context(),
            Term::Record(rec) => rec.to_context(),
            Term::RecordProj(proj) => proj.to_context(),
            Term::Left(lf) => lf.to_context(),
            Term::Right(rt) => rt.to_context(),
            Term::SumCase(case) => case.to_context(),
            Term::Variant(var) => var.to_context(),
            Term::VariantCase(case) => case.to_context(),
            Term::Nothing(not) => not.to_context(),
            Term::Something(some) => some.to_context(),
            Term::SomeCase(case) => case.to_context(),
            Term::Fix(fix) => fix.to_context(),
            Term::Nil(nil) => nil.to_context(),
            Term::Cons(cons) => cons.to_context(),
            Term::IsNil(isnil) => isnil.to_context(),
            Term::Head(hd) => hd.to_context(),
            Term::Tail(tl) => tl.to_context(),
        }
    }
}

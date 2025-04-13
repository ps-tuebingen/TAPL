use super::FreeVars;
use crate::syntax::Term;
use common::Var;
use std::collections::HashSet;

impl FreeVars for Term {
    fn free_vars(&self) -> HashSet<Var> {
        match self {
            Term::Var(v) => v.free_vars(),
            Term::Lambda(lam) => lam.free_vars(),
            Term::App(app) => app.free_vars(),
            Term::Unit(unit) => unit.free_vars(),
            Term::True(tru) => tru.free_vars(),
            Term::False(fls) => fls.free_vars(),
            Term::If(ift) => ift.free_vars(),
            Term::Zero(z) => z.free_vars(),
            Term::Succ(s) => s.free_vars(),
            Term::Pred(p) => p.free_vars(),
            Term::IsZero(isz) => isz.free_vars(),
            Term::Ascribe(asc) => asc.free_vars(),
            Term::Let(lt) => lt.free_vars(),
            Term::Pair(pr) => pr.free_vars(),
            Term::Proj1(proj) => proj.free_vars(),
            Term::Proj2(proj) => proj.free_vars(),
            Term::Tup(tup) => tup.free_vars(),
            Term::Proj(proj) => proj.free_vars(),
            Term::Record(rec) => rec.free_vars(),
            Term::RecordProj(proj) => proj.free_vars(),
            Term::Left(lft) => lft.free_vars(),
            Term::Right(rt) => rt.free_vars(),
            Term::SumCase(case) => case.free_vars(),
            Term::Variant(var) => var.free_vars(),
            Term::VariantCase(case) => case.free_vars(),
            Term::Nothing(not) => not.free_vars(),
            Term::Something(some) => some.free_vars(),
            Term::SomeCase(case) => case.free_vars(),
            Term::Fix(fix) => fix.free_vars(),
            Term::Nil(nil) => nil.free_vars(),
            Term::Cons(cons) => cons.free_vars(),
            Term::IsNil(isnil) => isnil.free_vars(),
            Term::Head(hd) => hd.free_vars(),
            Term::Tail(tl) => tl.free_vars(),
        }
    }
}

use super::Subst;
use crate::{syntax::Term, Var};

impl Subst for Term {
    type Target = Term;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        match self {
            Term::Var(v) => v.subst(var, term),
            Term::Lambda(lam) => lam.subst(var, term).into(),
            Term::App(app) => app.subst(var, term).into(),
            Term::Unit(unit) => unit.subst(var, term).into(),
            Term::True(tru) => tru.subst(var, term).into(),
            Term::False(fls) => fls.subst(var, term).into(),
            Term::If(ift) => ift.subst(var, term).into(),
            Term::Ascribe(asc) => asc.subst(var, term).into(),
            Term::Let(lt) => lt.subst(var, term).into(),
            Term::Pair(pr) => pr.subst(var, term).into(),
            Term::Proj1(proj) => proj.subst(var, term).into(),
            Term::Proj2(proj) => proj.subst(var, term).into(),
            Term::Tup(tup) => tup.subst(var, term).into(),
            Term::Proj(proj) => proj.subst(var, term).into(),
            Term::Record(rec) => rec.subst(var, term).into(),
            Term::RecordProj(proj) => proj.subst(var, term).into(),
            Term::Left(lf) => lf.subst(var, term).into(),
            Term::Right(rt) => rt.subst(var, term).into(),
            Term::SumCase(case) => case.subst(var, term).into(),
            Term::Variant(variant) => variant.subst(var, term).into(),
            Term::VariantCase(case) => case.subst(var, term).into(),
            Term::Nothing(not) => not.subst(var, term).into(),
            Term::Something(some) => some.subst(var, term).into(),
            Term::Fix(fix) => fix.subst(var, term).into(),
            Term::Nil(nil) => nil.subst(var, term).into(),
            Term::Cons(cons) => cons.subst(var, term).into(),
            Term::IsNil(isnil) => isnil.subst(var, term).into(),
            Term::Head(hd) => hd.subst(var, term).into(),
            Term::Tail(tl) => tl.subst(var, term).into(),
        }
    }
}

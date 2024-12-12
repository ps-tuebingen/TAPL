use super::{GenConstraints, GenState};
use crate::{syntax::Term, types::Type};

impl GenConstraints for Term {
    fn gen_constraints(self, st: &mut GenState) -> Type {
        match self {
            Term::Var(v) => v.gen_constraints(st),
            Term::Lambda(lam) => lam.gen_constraints(st),
            Term::App(app) => app.gen_constraints(st),
            Term::Unit(unit) => unit.gen_constraints(st),
            Term::True(tru) => tru.gen_constraints(st),
            Term::False(fls) => fls.gen_constraints(st),
            Term::If(ift) => ift.gen_constraints(st),
            Term::Zero(z) => z.gen_constraints(st),
            Term::Pred(p) => p.gen_constraints(st),
            Term::Succ(s) => s.gen_constraints(st),
            Term::IsZero(isz) => isz.gen_constraints(st),
            Term::Ascribe(asc) => asc.gen_constraints(st),
            Term::Let(lt) => lt.gen_constraints(st),
            Term::Pair(pr) => pr.gen_constraints(st),
            Term::Proj1(proj) => proj.gen_constraints(st),
            Term::Proj2(proj) => proj.gen_constraints(st),
            Term::Left(lf) => lf.gen_constraints(st),
            Term::Right(rt) => rt.gen_constraints(st),
            Term::SumCase(case) => case.gen_constraints(st),
            Term::Nothing(not) => not.gen_constraints(st),
            Term::Something(some) => some.gen_constraints(st),
            Term::SomeCase(case) => case.gen_constraints(st),
            Term::Fix(fix) => fix.gen_constraints(st),
            Term::Nil(nil) => nil.gen_constraints(st),
            Term::Cons(cons) => cons.gen_constraints(st),
            Term::IsNil(isnil) => isnil.gen_constraints(st),
            Term::Head(hd) => hd.gen_constraints(st),
            Term::Tail(tl) => tl.gen_constraints(st),
        }
    }
}

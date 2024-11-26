use super::{errors::Error, Check, TypingEnv};
use crate::{syntax::Term, types::Type};

impl Check for Term {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Ascribe(asc) => asc.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Pair(pr) => pr.check(env),
            Term::Proj1(proj) => proj.check(env),
            Term::Proj2(proj) => proj.check(env),
            Term::Tup(tup) => tup.check(env),
            Term::Proj(proj) => proj.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Left(lf) => lf.check(env),
            Term::Right(rt) => rt.check(env),
            Term::SumCase(case) => case.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Nothing(not) => not.check(env),
            Term::Something(some) => some.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::IsNil(isnil) => isnil.check(env),
            Term::Head(hd) => hd.check(env),
            Term::Tail(tl) => tl.check(env),
        }
    }
}

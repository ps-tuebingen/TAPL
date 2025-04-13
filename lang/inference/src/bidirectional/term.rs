use super::{Environment, Infer};
use crate::{syntax::Term, types::Type};
use common::errors::Error;

impl Infer for Term {
    fn infer(&self, env: &mut Environment) -> Result<Type, Error> {
        match self {
            Term::Var(v) => v.infer(env),
            Term::Lambda(lam) => lam.infer(env),
            Term::App(app) => app.infer(env),
            Term::Unit(unit) => unit.infer(env),
            Term::True(tru) => tru.infer(env),
            Term::False(fls) => fls.infer(env),
            Term::If(ift) => ift.infer(env),
            Term::Zero(z) => z.infer(env),
            Term::Pred(p) => p.infer(env),
            Term::Succ(s) => s.infer(env),
            Term::IsZero(isz) => isz.infer(env),
            Term::Ascribe(asc) => asc.infer(env),
            Term::Let(lt) => lt.infer(env),
            Term::Pair(pr) => pr.infer(env),
            Term::Proj1(proj) => proj.infer(env),
            Term::Proj2(proj) => proj.infer(env),
            Term::Left(lf) => lf.infer(env),
            Term::Right(rt) => rt.infer(env),
            Term::SumCase(case) => case.infer(env),
            Term::Nothing(not) => not.infer(env),
            Term::SomeCase(case) => case.infer(env),
            Term::Something(some) => some.infer(env),
            Term::Fix(fix) => fix.infer(env),
            Term::Nil(nil) => nil.infer(env),
            Term::Cons(cons) => cons.infer(env),
            Term::IsNil(isnil) => isnil.infer(env),
            Term::Head(hd) => hd.infer(env),
            Term::Tail(tl) => tl.infer(env),
        }
    }
    fn check(&self, target: Type, env: &mut Environment) -> Result<(), Error> {
        match self {
            Term::Var(v) => v.check(target, env),
            Term::Lambda(lam) => lam.check(target, env),
            Term::App(app) => app.check(target, env),
            Term::Unit(unit) => unit.check(target, env),
            Term::True(tru) => tru.check(target, env),
            Term::False(fls) => fls.check(target, env),
            Term::If(ift) => ift.check(target, env),
            Term::Zero(z) => z.check(target, env),
            Term::Pred(p) => p.check(target, env),
            Term::Succ(s) => s.check(target, env),
            Term::IsZero(isz) => isz.check(target, env),
            Term::Ascribe(asc) => asc.check(target, env),
            Term::Let(lt) => lt.check(target, env),
            Term::Pair(pr) => pr.check(target, env),
            Term::Proj1(proj) => proj.check(target, env),
            Term::Proj2(proj) => proj.check(target, env),
            Term::Left(lf) => lf.check(target, env),
            Term::Right(rt) => rt.check(target, env),
            Term::SumCase(case) => case.check(target, env),
            Term::Nothing(not) => not.check(target, env),
            Term::Something(some) => some.check(target, env),
            Term::SomeCase(case) => case.check(target, env),
            Term::Fix(fix) => fix.check(target, env),
            Term::Nil(nil) => nil.check(target, env),
            Term::Cons(cons) => cons.check(target, env),
            Term::IsNil(isnil) => isnil.check(target, env),
            Term::Head(hd) => hd.check(target, env),
            Term::Tail(tl) => tl.check(target, env),
        }
    }
}

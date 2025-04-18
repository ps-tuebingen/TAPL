use super::{terms::Term, types::Type};
use common::{
    check::{Subtypecheck, Typecheck},
    errors::Error,
    Var,
};
use std::collections::HashMap;

impl Typecheck for Term {
    type Type = Type;
    type Env = HashMap<Var, Type>;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Num(num) => num.check(env),
            Term::Pred(p) => p.check(env),
            Term::Succ(s) => s.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Ascribe(asc) => asc.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Pair(pr) => pr.check(env),
            Term::Tuple(tup) => tup.check(env),
            Term::Projection(proj) => proj.check(env),
            Term::Fst(proj) => proj.check(env),
            Term::Snd(proj) => proj.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Left(lf) => lf.check(env),
            Term::Right(rt) => rt.check(env),
            Term::SumCase(case) => case.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Nothing(not) => not.check(env),
            Term::Something(some) => some.check(env),
            Term::SomeCase(case) => case.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::IsNil(isnil) => isnil.check(env),
            Term::Head(hd) => hd.check(env),
            Term::Tail(tl) => tl.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = HashMap<Var, Type>;
    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

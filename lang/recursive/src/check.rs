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
            Term::Unit(u) => u.check(env),
            Term::Fold(fold) => fold.check(env),
            Term::Unfold(unfold) => unfold.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Pair(p) => p.check(env),
            Term::Fst(fst) => fst.check(env),
            Term::Snd(snd) => snd.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
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

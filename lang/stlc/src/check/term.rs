use super::{to_check_err, TypingEnv};
use crate::{syntax::Term, types::Type};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env
                .used_vars
                .get(v)
                .cloned()
                .ok_or(to_check_err(ErrorKind::FreeVariable(v.clone()))),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Zero(z) => z.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
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

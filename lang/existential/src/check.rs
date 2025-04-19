use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    errors::Error,
    TypeVar, Var,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Default)]
pub struct Env {
    vars: HashMap<Var, Type>,
    ty_vars: HashSet<TypeVar>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    fn get_var(&self, v: &Var) -> Result<Type, Error> {
        self.vars.get_var(v)
    }

    fn add_var(&mut self, v: Var, ty: Type) {
        self.vars.add_var(v, ty);
    }
}

impl Typecheck for Term {
    type Env = Env;
    type Type = Type;

    fn check(&self, env: &mut Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Unit(u) => u.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = Env;
    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

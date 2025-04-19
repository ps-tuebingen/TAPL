use crate::{terms::Term, types::Type};
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
            Term::Var(var) => var.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::LambdaSub(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::Projection(proj) => proj.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = HashMap<Var, Type>;

    fn check_subtype(&self, sup: &Self, env: &mut Self::Env) -> Result<(), Error> {
        match self {
            Type::Var(var) => var.check_subtype(sup, env),
            Type::Top(t) => t.check_subtype(sup, env),
            Type::Nat(n) => n.check_subtype(sup, env),
            Type::Fun(f) => f.check_subtype(sup, env),
            Type::Forall(f) => f.check_subtype(sup, env),
            Type::Exists(e) => e.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
        }
    }

    fn check_supertype(&self, sub: &Self, env: &mut Self::Env) -> Result<(), Error> {
        match self {
            Type::Var(var) => var.check_supertype(sub, env),
            Type::Top(t) => t.check_supertype(sub, env),
            Type::Nat(n) => n.check_supertype(sub, env),
            Type::Fun(f) => f.check_supertype(sub, env),
            Type::Forall(f) => f.check_supertype(sub, env),
            Type::Exists(e) => e.check_supertype(sub, env),
            Type::Record(rec) => rec.check_supertype(sub, env),
        }
    }
}

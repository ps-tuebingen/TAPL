use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    errors::Error,
    TypeVar, Var,
};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: Vec<TypeVar>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        self.vars.get_var(v)
    }
    fn add_var(&mut self, v: Var, ty: Self::Type) {
        self.vars.add_var(v, ty)
    }
}

impl Typecheck for Term {
    type Env = Env;
    type Type = Type;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = Env;
    fn check_subtype(&self, _: &Type, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }

    fn check_supertype(&self, _: &Type, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

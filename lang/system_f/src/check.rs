use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    Location, TypeVar, Var,
};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    fn get_var(&self, v: &Var) -> Result<Self::Type, ErrorKind> {
        self.vars.get_var(v)
    }
    fn add_var(&mut self, v: Var, ty: Self::Type) {
        self.vars.add_var(v, ty)
    }
    fn get_loc(&self, loc: &Location) -> Result<Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(*loc))
    }

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        self.ty_vars
            .get(v)
            .ok_or(ErrorKind::FreeTypeVariable(v.clone()))
            .cloned()
    }

    fn add_tyvar_kind(&mut self, v: TypeVar, knd: Kind) {
        self.ty_vars.insert(v, knd);
    }

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}
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

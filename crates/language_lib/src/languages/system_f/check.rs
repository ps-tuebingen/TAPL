use super::{errors::Error, terms::Term, types::Type};
use check::{errors::FreeTypeVariable, CheckEnvironment, Kindcheck, Subtypecheck, Typecheck};
use common::errors::NotImplemented;
use std::collections::HashMap;
use syntax::{kinds::Kind, Location, TypeVar, Var};

#[derive(Default, Clone)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    type CheckError = Error;

    fn get_var(&self, v: &Var) -> Result<Self::Type, Error> {
        self.vars.get_var(v).map_err(|err| err.into())
    }
    fn add_var(&mut self, v: Var, ty: Self::Type) {
        self.vars.add_var(v, ty)
    }
    fn get_loc(&self, _: &Location) -> Result<Type, Error> {
        Err(NotImplemented.into())
    }

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, Error> {
        self.ty_vars
            .get(v)
            .ok_or(FreeTypeVariable::new(v).into())
            .cloned()
    }

    fn add_tyvar_kind(&mut self, v: TypeVar, knd: Kind) {
        self.ty_vars.insert(v, knd);
    }

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, Error> {
        Err(FreeTypeVariable::new(v).into())
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}
}

impl Typecheck for Term {
    type Env = Env;
    type Type = Type;
    type CheckError = Error;

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
    type CheckError = Error;

    fn check_subtype(&self, _: &Type, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    type CheckError = Error;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

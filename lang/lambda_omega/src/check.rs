use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    Location, TypeVar, Var,
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl CheckEnvironment for Env {
    type Type = Type;

    fn add_tyvar_kind(&mut self, var: TypeVar, kind: Kind) {
        self.ty_vars.insert(var, kind);
    }

    fn get_tyvar_kind(&self, var: &TypeVar) -> Result<Kind, ErrorKind> {
        self.ty_vars
            .get(var)
            .ok_or(ErrorKind::FreeTypeVariable(var.clone()))
            .cloned()
    }

    fn add_var(&mut self, var: Var, ty: Type) {
        self.vars.add_var(var, ty)
    }

    fn get_var(&self, var: &Var) -> Result<Type, ErrorKind> {
        self.vars.get_var(var)
    }

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::FreeTypeVariable(v.clone()))
    }

    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(*loc))
    }
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Num(num) => num.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(u) => u.check(env),
            Term::TyLambda(tylam) => tylam.check(env),
            Term::TyApp(tyapp) => tyapp.check(env),
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

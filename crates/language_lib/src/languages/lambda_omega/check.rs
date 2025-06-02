use super::{errors::Error, terms::Term, types::Type};
use check::{errors::FreeTypeVariable, CheckEnvironment, Kindcheck, Subtypecheck, Typecheck};
use common::errors::NotImplemented;
use std::collections::HashMap;
use syntax::{kinds::Kind, Location, TypeVar, Var};

#[derive(Clone, Default)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    type CheckError = Error;

    fn add_tyvar_kind(&mut self, var: TypeVar, kind: Kind) {
        self.ty_vars.insert(var, kind);
    }

    fn get_tyvar_kind(&self, var: &TypeVar) -> Result<Kind, Self::CheckError> {
        self.ty_vars
            .get(var)
            .ok_or(FreeTypeVariable::new(var).into())
            .cloned()
    }

    fn add_var(&mut self, var: Var, ty: Type) {
        self.vars.add_var(var, ty)
    }

    fn get_var(&self, var: &Var) -> Result<Type, Self::CheckError> {
        self.vars.get_var(var).map_err(|err| err.into())
    }

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }

    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_loc(&self, loc: &Location) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Env;
    type CheckError = Error;

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
    type CheckError = Error;

    fn check_subtype(&self, _: &Type, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    type CheckError = Error;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        match self {
            Type::Var(var) => var.check_kind(env),
            Type::Unit(u) => u.check_kind(env),
            Type::Bool(b) => b.check_kind(env),
            Type::Nat(n) => n.check_kind(env),
            Type::OpLambda(oplam) => oplam.check_kind(env),
            Type::OpApp(opapp) => opapp.check_kind(env),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
        }
    }
}

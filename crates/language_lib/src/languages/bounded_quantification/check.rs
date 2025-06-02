use super::{errors::Error, terms::Term, types::Type};
use check::{env::CheckEnvironment, errors::FreeTypeVariable, Kindcheck, Subtypecheck, Typecheck};
use common::errors::NotImplemented;
use std::collections::HashMap;
use syntax::{kinds::Kind, Location, TypeVar, Var};

#[derive(Clone, Default)]
pub struct Env {
    vars: HashMap<Var, Type>,
    ty_vars: HashMap<TypeVar, Type>,
}

impl CheckEnvironment for Env {
    type Type = Type;
    type CheckError = Error;
    fn get_var(&self, v: &Var) -> Result<Self::Type, Self::CheckError> {
        self.vars.get_var(v).map_err(|err| err.into())
    }

    fn add_var(&mut self, v: Var, ty: Self::Type) {
        self.vars.add_var(v, ty)
    }

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, Self::CheckError> {
        self.ty_vars
            .get(v)
            .map(|_| Kind::Star)
            .ok_or(FreeTypeVariable::new(&v).into())
    }

    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        self.ty_vars
            .get(v)
            .ok_or(FreeTypeVariable::new(v).into())
            .cloned()
    }

    fn add_tyvar_super(&mut self, v: TypeVar, ty: Self::Type) {
        self.ty_vars.insert(v, ty);
    }

    fn get_loc(&self, _: &Location) -> Result<Self::Type, Self::CheckError> {
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
    type Env = Env;
    type CheckError = Error;

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
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    type CheckError = Error;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

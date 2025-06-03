use super::{errors::Error, terms::Term, types::Type};
use check::{CheckEnvironment, Kindcheck, Subtypecheck, Typecheck};
use common::errors::{FreeTypeVariable, NotImplemented};
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

    fn get_var(&self, v: &Var) -> Result<Type, Self::CheckError> {
        self.vars.get_var(v).map_err(|err| err.into())
    }

    fn add_var(&mut self, v: Var, ty: Type) {
        self.vars.add_var(v, ty)
    }

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, Self::CheckError> {
        self.ty_vars
            .get(v)
            .cloned()
            .ok_or(FreeTypeVariable::new(v).into())
    }

    fn add_tyvar_kind(&mut self, v: TypeVar, kind: Kind) {
        self.ty_vars.insert(v, kind);
    }

    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}
    fn get_tyvar_super(&self, _: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
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
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Unit(u) => u.check(env),
            Term::Fix(fix) => fix.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::IsZero(isz) => isz.check(env),
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
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
            Type::OpLambda(lam) => lam.check_kind(env),
            Type::OpApp(app) => app.check_kind(env),
            Type::Exists(ex) => ex.check_kind(env),
            Type::Record(rec) => rec.check_kind(env),
            Type::Bool(b) => b.check_kind(env),
            Type::Unit(u) => u.check_kind(env),
            Type::Nat(nat) => nat.check_kind(env),
        }
    }
}

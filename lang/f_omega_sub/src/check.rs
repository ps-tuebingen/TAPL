use super::{terms::Term, types::Type};
use common::{
    check::{CheckEnvironment, Kindcheck, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    types::Top,
    Location, TypeVar, Var,
};
use std::collections::HashMap;

#[derive(Clone, Default)]
pub struct Env {
    vars: HashMap<Var, Type>,
    ty_vars: HashMap<TypeVar, Type>,
}

impl CheckEnvironment for Env {
    type Type = Type;

    fn add_var(&mut self, v: Var, ty: Type) {
        self.vars.add_var(v, ty)
    }

    fn get_var(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.vars.get_var(v)
    }

    fn add_tyvar_super(&mut self, v: TypeVar, sup_ty: Type) {
        self.ty_vars.insert(v, sup_ty);
    }

    fn get_tyvar_super(&self, v: &TypeVar) -> Result<Type, ErrorKind> {
        self.ty_vars
            .get(v)
            .ok_or(ErrorKind::FreeTypeVariable(v.clone()))
            .cloned()
    }

    fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        let sup = self
            .ty_vars
            .get(v)
            .ok_or(ErrorKind::FreeTypeVariable(v.clone()))?;
        let sup_kind = sup.check_kind(&mut self.clone()).map_err(|err| err.kind)?;
        Ok(sup_kind)
    }

    fn add_tyvar_kind(&mut self, v: TypeVar, kind: Kind) {
        self.ty_vars.insert(v, Top::new(kind).into());
    }

    fn get_loc(&self, loc: &Location) -> Result<Type, ErrorKind> {
        Err(ErrorKind::UndefinedLocation(loc.clone()))
    }
}

impl Typecheck for Term {
    type Type = Type;
    type Env = Env;

    fn check(&self, env: &mut Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::LambdaSub(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::Let(lt) => lt.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = Env;

    fn check_subtype(&self, sup: &Type, env: &mut Self::Env) -> Result<(), Error> {
        match self {
            Type::Var(var) => var.check_subtype(sup, env),
            Type::Top(top) => top.check_subtype(sup, env),
            Type::Fun(fun) => fun.check_subtype(sup, env),
            Type::Forall(forall) => forall.check_subtype(sup, env),
            Type::OpLambda(lam) => lam.check_subtype(sup, env),
            Type::OpApp(app) => app.check_subtype(sup, env),
            Type::Exists(ex) => ex.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
            Type::Nat(nat) => nat.check_subtype(sup, env),
        }
    }
}

impl Kindcheck<Type> for Type {
    type Env = Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        match self {
            Type::Var(var) => var.check_kind(env),
            Type::Top(top) => top.check_kind(env),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Forall(forall) => forall.check_kind(env),
            Type::OpLambda(lam) => lam.check_kind(env),
            Type::OpApp(app) => app.check_kind(env),
            Type::Exists(ex) => ex.check_kind(env),
            Type::Record(rec) => rec.check_kind(env),
            Type::Nat(nat) => nat.check_kind(env),
        }
    }
}

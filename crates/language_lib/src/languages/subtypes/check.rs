use super::{errors::Error, terms::Term, types::Type};
use check::{CheckEnvironment, Kindcheck, Subtypecheck, Typecheck};
use common::errors::{NotImplemented, UndefinedLocation};
use std::collections::HashMap;
use syntax::{kinds::Kind, Location, TypeVar, Var};

#[derive(Clone, Default)]
pub struct TypingContext {
    var_env: HashMap<Var, Type>,
    store_typing: HashMap<Location, Type>,
}

impl CheckEnvironment for TypingContext {
    type Type = Type;
    type CheckError = Error;

    fn get_var(&self, v: &Var) -> Result<Type, Self::CheckError> {
        self.var_env.get_var(v).map_err(|err| err.into())
    }

    fn add_var(&mut self, var: Var, ty: Type) {
        self.var_env.insert(var, ty);
    }

    fn get_loc(&self, loc: &Location) -> Result<Type, Self::CheckError> {
        self.store_typing
            .get(loc)
            .ok_or(UndefinedLocation::new(*loc).into())
            .cloned()
    }

    fn get_tyvar_super(&self, _: &TypeVar) -> Result<Self::Type, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_super(&mut self, _: TypeVar, _: Self::Type) {}

    fn get_tyvar_kind(&self, _: &TypeVar) -> Result<Kind, Self::CheckError> {
        Err(NotImplemented.into())
    }
    fn add_tyvar_kind(&mut self, _: TypeVar, _: Kind) {}
}

impl Typecheck for Term {
    type Env = TypingContext;
    type Type = Type;
    type CheckError = Error;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Cast(cast) => cast.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::ListCase(case) => case.check(env),
            Term::Ref(rf) => rf.check(env),
            Term::Deref(deref) => deref.check(env),
            Term::Assign(assign) => assign.check(env),
            Term::Loc(loc) => loc.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type Env = TypingContext;
    type CheckError = Error;

    fn check_subtype(&self, sup: &Self, env: &mut Self::Env) -> Result<(), Error> {
        match self {
            Type::Top(top) => top.check_subtype(sup, env),
            Type::Bot(bot) => bot.check_subtype(sup, env),
            Type::Fun(fun) => fun.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
            Type::Variant(variant) => variant.check_subtype(sup, env),
            Type::List(list) => list.check_subtype(sup, env),
            Type::Ref(refty) => refty.check_subtype(sup, env),
            Type::Source(src) => src.check_subtype(sup, env),
            Type::Sink(snk) => snk.check_subtype(sup, env),
            Type::Nat(nat) => nat.check_subtype(sup, env),
            Type::Unit(unit) => unit.check_subtype(sup, env),
            Type::Bool(b) => b.check_subtype(sup, env),
        }
    }
}

impl Kindcheck<Type> for Type {
    type Env = TypingContext;
    type CheckError = Error;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

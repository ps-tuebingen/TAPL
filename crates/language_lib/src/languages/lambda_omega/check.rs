use super::{errors::Error, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type CheckError = Error;

    fn check(
        &self,
        env: &mut Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Error> {
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
    type CheckError = Error;

    fn check_subtype(&self, _: &Type, _: &mut Environment<Type>) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type CheckError = Error;

    fn check_kind(&self, env: &mut Environment<Type>) -> Result<Kind, Error> {
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

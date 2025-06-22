use super::{terms::Term, types::Type};
use check::{errors::CheckError, Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Type>> {
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
    fn check_subtype(&self, _: &Type, _: Environment<Type>) -> Result<(), CheckError<Type>> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, env: Environment<Type>) -> Result<Kind, CheckError<Type>> {
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

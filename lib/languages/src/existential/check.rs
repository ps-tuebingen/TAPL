use super::{terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Unit(u) => u.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    fn check_subtype(&self, _: &Self, _: Environment<Type>) -> Result<(), CheckError> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

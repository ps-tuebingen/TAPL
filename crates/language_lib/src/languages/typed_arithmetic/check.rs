use super::{errors::Error, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type CheckError = Error;

    fn check(&self, env: Environment<Type>) -> Result<Derivation<Self::Term, Self::Type>, Error> {
        match self {
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::If(ift) => ift.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    type CheckError = Error;

    fn check_subtype(&self, _: &Self, _: Environment<Type>) -> Result<(), Error> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    type CheckError = Error;

    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

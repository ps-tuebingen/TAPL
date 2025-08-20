use super::{terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoSubtyping, check_error::CheckError};
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
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

impl Subtypecheck for Type {
    type Type = Type;
    type Term = Term;
    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        Err(NoSubtyping::new("Typed Arithmetic").into())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

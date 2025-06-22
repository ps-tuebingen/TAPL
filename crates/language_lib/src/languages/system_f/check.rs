use super::{terms::Term, types::Type};
use check::{errors::CheckError, Kindcheck, Subtypecheck, Typecheck};
use derivation::TypingDerivation;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<TypingDerivation<Self::Term, Self::Type>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    fn check_subtype(&self, _: &Type, _: Environment<Type>) -> Result<(), CheckError> {
        Ok(())
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

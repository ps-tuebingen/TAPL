use crate::{Kindcheck, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck for Bot<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Bot<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        Ok(SubtypeDerivation::sup_bot(env, sup.clone()).into())
    }
}

impl<Ty> Kindcheck<Ty> for Bot<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(self.kind.clone())
    }
}

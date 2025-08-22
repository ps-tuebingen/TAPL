use crate::{Kindcheck, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck for Bot<Ty>
where
    Ty: TypeGroup + Subtypecheck,
    Bot<Ty>: Into<Ty>,
{
    type Lang = <Ty as Subtypecheck>::Lang;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<
        Derivation<<Self::Lang as Language>::Term, <Self::Lang as Language>::Type>,
        CheckError,
    > {
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

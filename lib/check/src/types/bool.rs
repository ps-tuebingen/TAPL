use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bool, Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck for Bool<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Top<Ty>: Into<Ty>,
    Self: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        sup.clone().into_bool()?;
        Ok(SubtypeDerivation::refl(env, self.clone()).into())
    }
}

impl<Ty> Kindcheck<Ty> for Bool<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Bool<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}

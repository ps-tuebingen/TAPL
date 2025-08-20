use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::{NotASubtype, check_error::CheckError};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck for Top<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty> + From<Self>,
    Top<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into())
        } else {
            Err(NotASubtype::new(self.clone(), sup.clone()).into())
        }
    }
}

impl<Ty> Kindcheck<Ty> for Top<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(self.kind.clone())
    }
}

impl<Ty> Normalize<Ty> for Top<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}

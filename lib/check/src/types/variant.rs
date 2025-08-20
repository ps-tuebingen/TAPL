use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::{UndefinedLabel, check_error::CheckError};
use syntax::{
    env::Environment,
    types::{Top, TypeGroup, Variant},
};

impl<Ty> Subtypecheck for Variant<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Top<Ty>: Into<Ty>,
    Variant<Ty>: Into<Ty>,
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

        let sup_var = sup.clone().into_variant()?;
        let mut inner_res = vec![];
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self.variants.get(lb).ok_or(UndefinedLabel::new(lb))?;
            inner_res.push(self_ty.check_subtype(ty, env.clone())?);
        }
        Ok(SubtypeDerivation::variant(env, self.clone(), sup.clone(), inner_res).into())
    }
}

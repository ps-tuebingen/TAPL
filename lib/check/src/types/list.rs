use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    types::{List, Top, TypeGroup},
};

impl<Ty> Subtypecheck for List<Ty>
where
    Ty: TypeGroup + Subtypecheck,
    Top<Ty>: Into<Ty>,
    List<Ty>: Into<Ty>,
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
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_list = sup.clone().into_list()?;
        let sup_res = self.ty.check_subtype(&(*sup_list.ty), env.clone())?;
        Ok(SubtypeDerivation::list(env, self.clone(), sup.clone(), sup_res).into())
    }
}

use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    types::{Source, Top, TypeGroup},
};

impl<Ty> Subtypecheck for Source<Ty>
where
    Ty: TypeGroup + Subtypecheck,
    Top<Ty>: Into<Ty>,
    Source<Ty>: Into<Ty>,
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

        let sup_src = sup.clone().into_source()?;
        let inner_res = self.ty.check_subtype(&(*sup_src.ty), env.clone())?;
        Ok(SubtypeDerivation::source(env, self.clone(), sup_src, inner_res).into())
    }
}

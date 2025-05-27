use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    subst::SubstType,
    terms::{Term, Unfold},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Unfold<T, Ty>
where
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + SubstType<Ty, Target = Ty>,
    T: Term + Typecheck<Type = Ty>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let ty_kind = ty_norm.check_kind(&mut env.clone())?;

        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        term_knd.check_equal(&ty_kind).map_err(to_check_err)?;

        ty_norm.check_equal(&term_ty).map_err(to_check_err)?;
        let mu_ty = term_ty.clone().into_mu().map_err(to_check_err)?;
        Ok(mu_ty.ty.subst_type(&mu_ty.var, &term_ty))
    }
}

use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{ListCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for ListCase<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let bound_list = bound_ty.clone().into_list().map_err(to_check_err)?;

        let nil_ty = self
            .nil_rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let nil_kind = nil_ty.check_kind(&mut env.clone())?;

        env.add_var(self.cons_fst.clone(), *bound_list.ty);
        env.add_var(self.cons_rst.clone(), bound_ty);
        let cons_ty = self
            .cons_rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let cons_kind = cons_ty.check_kind(env)?;

        nil_kind.check_equal(&cons_kind).map_err(to_check_err)?;
        nil_ty.check_equal(&cons_ty).map_err(to_check_err)?;
        Ok(cons_ty)
    }
}

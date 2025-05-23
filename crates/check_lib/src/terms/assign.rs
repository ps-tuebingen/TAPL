impl<T> Typecheck for Assign<T>
where
    T: LanguageTerm,
    UnitTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let lhs_ty = self
            .lhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        lhs_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let lhs_ref = lhs_ty.into_ref().map_err(to_check_err)?;

        let rhs_ty = self
            .rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        rhs_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        lhs_ref.ty.check_equal(&rhs_ty).map_err(to_check_err)?;
        Ok(UnitTy::new().into())
    }
}

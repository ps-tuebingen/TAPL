impl<T> Typecheck for IsZero<T>
where
    T: LanguageTerm,
    Bool<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        inner_ty
            .check_kind(env)?
            .into_star()
            .map_err(to_check_err)?;
        inner_ty.into_nat().map_err(to_check_err)?;
        Ok(Bool::new().into())
    }
}

impl<T> Typecheck for IsNil<T>
where
    T: LanguageTerm,
    List<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
    Bool<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        term_ty.into_list().map_err(to_check_err)?;
        Ok(Bool::new().into())
    }
}

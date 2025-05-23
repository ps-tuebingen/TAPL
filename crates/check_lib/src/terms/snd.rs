impl<T> Typecheck for Snd<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?;
        let prod_ty = term_ty.into_product().map_err(to_check_err)?;
        Ok(*prod_ty.snd)
    }
}

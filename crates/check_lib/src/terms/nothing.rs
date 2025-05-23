impl<T> Typecheck for Nothing<T>
where
    T: LanguageTerm,
    Optional<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Optional::new(ty_norm.clone()).into())
    }
}

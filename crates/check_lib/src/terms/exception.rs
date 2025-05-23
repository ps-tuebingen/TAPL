impl<T> Typecheck for Exception<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?;
        Ok(ty_norm)
    }
}

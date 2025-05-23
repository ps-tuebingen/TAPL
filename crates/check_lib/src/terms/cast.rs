impl<T> Typecheck for Cast<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let term_kind = term_ty.check_kind(&mut env.clone())?;
        let ty_kind = ty_norm.check_kind(env)?;
        term_kind.check_equal(&ty_kind).map_err(to_check_err)?;
        Ok(ty_norm)
    }
}

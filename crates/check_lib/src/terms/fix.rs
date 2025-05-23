impl<T> Typecheck for Fix<T>
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
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let fun_ty = term_ty.into_fun().map_err(to_check_err)?;
        fun_ty.from.check_equal(&fun_ty.to).map_err(to_check_err)?;
        Ok(*fun_ty.from)
    }
}

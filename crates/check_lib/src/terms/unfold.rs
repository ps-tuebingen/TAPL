impl<T> Typecheck for Unfold<T>
where
    T: LanguageTerm,
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

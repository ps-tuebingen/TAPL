impl<T> Typecheck for Fold<T>
where
    T: LanguageTerm,
    Mu<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let mu_ty = self
            .ty
            .clone()
            .normalize(&mut env.clone())
            .into_mu()
            .map_err(to_check_err)?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        mu_ty
            .ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        term_ty.check_equal(&mu_subst).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

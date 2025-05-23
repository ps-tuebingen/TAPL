impl<T> Typecheck for LambdaSub<T>
where
    T: LanguageTerm,
    ForallBounded<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let sup_norm = self.sup_ty.clone().normalize(&mut env.clone());
        let sup_kind = sup_norm.check_kind(&mut env.clone())?;
        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let term_ty = self.body.check(&mut env.clone())?.normalize(env);
        Ok(ForallBounded::new(&self.var, self.sup_ty.clone(), term_ty).into())
    }
}

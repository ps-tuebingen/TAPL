impl<T> Typecheck for Let<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?;

        env.add_var(self.var.clone(), bound_ty);
        let in_ty = self
            .in_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        in_ty.check_kind(env)?;
        Ok(in_ty)
    }
}

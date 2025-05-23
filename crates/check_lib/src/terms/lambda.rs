impl<T> Typecheck for Lambda<T>
where
    T: LanguageTerm,
    Fun<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        self.annot.check_kind(&mut env.clone())?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_ty = self
            .body
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        body_ty.check_kind(&mut env.clone())?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}

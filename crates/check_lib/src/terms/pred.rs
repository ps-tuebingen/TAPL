impl<T> Typecheck for Pred<T>
where
    T: LanguageTerm,
    Nat<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(&mut env.clone())?.normalize(env);
        let nat = inner_ty.into_nat().map_err(to_check_err)?;
        Ok(nat.into())
    }
}

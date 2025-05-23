impl<T> Typecheck for Pair<T>
where
    T: LanguageTerm,
    Product<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let fst_ty = self
            .fst
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let snd_ty = self
            .snd
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let fst_knd = fst_ty.check_kind(&mut env.clone())?;
        let snd_knd = snd_ty.check_kind(env)?;
        fst_knd.check_equal(&snd_knd).map_err(to_check_err)?;
        Ok(Product::new(fst_ty, snd_ty).into())
    }
}
